// use std::os::raw::*;
use std::ffi::*;
use crate::bindings::{commit, commit_extra_header, commit_list, commit_list_insert, commit_tree_extended, find_commit_header, find_commit_subject, free, free_commit_extra_headers, free_commit_list, get_commit_output_encoding, object, object_id, parse_object, read_commit_extra_headers, repo_logmsg_reencode, repo_unuse_commit_buffer, repository, reset_ident_date, strbuf, strbuf_add, strbuf_release, strbuf_slopbuf, strlen, the_repository, tree, xmemdupz};

unsafe fn get_author(message: *const c_char) -> *mut c_char {
	let mut len: usize = 0;
	// const char *a;
	let mut a: *const c_char;

	a = find_commit_header(message, "author".as_ptr() as *const c_char, &mut len);
	if (!a.is_null()) {
		return xmemdupz(a as *const c_void, len) as *mut c_char;
	}

	std::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn create_commit(repo: *mut repository, tree_: *mut tree, based_on: *mut commit, parent: *mut commit) -> *mut commit {

	// struct object_id ret;
	let mut ret: object_id = object_id{ hash: [0; 32], algo: 0 };
	// struct object *obj = NULL;
	let mut obj: *mut object = std::ptr::null_mut();
	// struct commit_list *parents = NULL;
    let mut parents: *mut commit_list = std::ptr::null_mut();
	// char *author;
	let author: *mut c_char;
	// char *sign_commit = NULL; /* FIXME: cli users might want to sign again */
	let sign_commit: *mut c_char = std::ptr::null_mut();
	// struct commit_extra_header *extra = NULL;
	let mut extra: *mut commit_extra_header = std::ptr::null_mut();
	// struct strbuf msg = STRBUF_INIT;
	let mut msg: strbuf = strbuf{
		alloc: 0,
		len: 0,
		buf: [0u8; 0].as_ptr() as *mut c_char,
	};
	// const char *out_enc = get_commit_output_encoding();
	let out_enc = get_commit_output_encoding();
	// const char *message = repo_logmsg_reencode(repo, based_on,
	// 					   NULL, out_enc);
	let message: *const c_char = repo_logmsg_reencode(repo, based_on, std::ptr::null_mut(), out_enc);
	// const char *orig_message = NULL;
	let mut orig_message: *const c_char = std::ptr::null_mut();
	// const char *exclude_gpgsig[] = { "gpgsig", NULL };
	let mut exclude_gpgsig: Vec<*const c_char> = vec![b"gpgsig".as_ptr() as *const c_char, std::ptr::null_mut()];

	commit_list_insert(parent, &mut parents);
	extra = read_commit_extra_headers(based_on, exclude_gpgsig.as_mut_ptr());
	find_commit_subject(message, &mut orig_message);
	// strbuf_addstr(&msg, orig_message);
	strbuf_add(&mut msg, orig_message as *mut c_void, strlen(orig_message) as usize);
	author = get_author(message);
	reset_ident_date();
	if (commit_tree_extended(msg.buf, msg.len, &(*tree_).object.oid, parents,
				 &mut ret, author, std::ptr::null_mut(), sign_commit, extra) != 0) {
		panic!("failed to write commit object");
		// error(_("failed to write commit object"));
		// goto out;
	}

	obj = parse_object(repo, &ret);

// out:
	repo_unuse_commit_buffer(the_repository, based_on, message as *mut c_void);
	free_commit_extra_headers(extra);
	free_commit_list(parents);
	strbuf_release(&mut msg);
	free(author as *mut c_void);

	obj as *mut commit
}