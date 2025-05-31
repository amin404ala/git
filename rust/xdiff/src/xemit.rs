use std::fmt::Write;
use interop::ivec::IVec;
use crate::xdiff::{mmbuffer, xdemitcb, xdemitcb_out_line_func, xdemitconf};
use crate::xdiffi::xdchange;
use crate::xtypes::xdpair;

pub(crate) type emit_func_t = unsafe extern "C" fn(pair: *mut xdpair, xscr: *mut xdchange, ecb: *const xdemitcb,
			   xecfg: *const xdemitconf) -> i32;



/// # Safety
///
/// The caller must ensure that `out` has at least 21 bytes available.
/// 21 because -2**63 as a string is 20 bytes + null byte.
#[no_mangle]
unsafe extern "C" fn xdl_num_out(out: *mut u8, mut val: i64) -> usize {
	let t = val.to_string();
	let raw = t.as_bytes();
	let dst = unsafe { std::slice::from_raw_parts_mut(out, 21) };
	dst[..raw.len()].copy_from_slice(raw);
	dst[raw.len()..].fill(0);

	raw.len()
}

#[no_mangle]
unsafe extern "C" fn append_i64(builder: *mut IVec<u8>, val: i64) {
	let builder = IVec::from_raw_mut(builder);
	write!(builder, "{}", val).unwrap();
}


#[no_mangle]
unsafe extern "C" fn xdl_format_hunk_hdr(
	s1: usize, c1: usize, s2: usize, c2: usize,
	func: *const u8, funclen: usize,
	ecb: *mut xdemitcb
) -> i32 {
	const MAX_WIDTH: usize = 128;
	let ecb = &mut *ecb;
	let mut builder = IVec::<u8>::new();
	
	write!(builder, "@@ -{}", if c1 != 0 { s1  } else { s1 - 1 }).unwrap();
	if c1 != 1 {
		write!(builder, ",{}", c1).unwrap();
	}
	
	write!(builder, " +{}", if c2 != 0 { s2 } else { s2 - 1 }).unwrap();
	if c2 != 1 {
		write!(builder, ",{}", c2).unwrap();
	}
	
	write!(builder, " @@").unwrap();
	if !func.is_null() && funclen != 0 {
		write!(builder, " ").unwrap();

		let slice = std::slice::from_raw_parts(func, funclen);
		let write = std::cmp::min(funclen, MAX_WIDTH - builder.len() - 1);
		builder.extend_from_slice(&slice[..write]);
	}
	builder.push(b'\n');

	let mut mb = mmbuffer::from_slice(builder.as_slice());
	
	if ecb.invoke_out_line(&mut mb, 1) < 0 {
		return -1;
	}
	
	0
}


#[no_mangle]
unsafe extern "C" fn xdl_emit_hunk_hdr(s1: usize, c1: usize, s2: usize, c2: usize,
									   func: *const u8, funclen: usize,
									   ecb: *mut xdemitcb) -> i32 {
	let ecb = &mut *ecb;
	if ecb.is_out_hunk_null() {
		return xdl_format_hunk_hdr(s1, c1, s2, c2, func, funclen, ecb);
	}
	let old_begin = if c1 != 0 { s1 } else { s1 - 1 };
	let new_begin = if c2 != 0 { s2 } else { s2 - 1 };
	if ecb.invoke_out_hunk(old_begin as isize, c1 as isize,
							new_begin as isize, c2 as isize,
		func, funclen as isize) < 0 {
		return -1;
	}
	
	0
}


/*
 * Starting at the passed change atom, find the latest change atom to be included
 * inside the differential hunk according to the specified configuration.
 * Also advance xscr if the first changes must be discarded.
 */
#[no_mangle]
pub(crate) unsafe extern "C" fn xdl_get_hunk(xscr: *mut *mut xdchange, xecfg: *const xdemitconf) -> *mut xdchange {
	let max_common = 2 * (*xecfg).ctxlen + (*xecfg).interhunkctxlen;
	let max_ignorable = (*xecfg).ctxlen;
	let mut ignored = 0; /* number of ignored blank lines */

	/* remove ignorable changes that are too far before other changes */
	let mut xchp = *xscr;
	let mut xch: *mut xdchange = std::ptr::null_mut();
	while !xchp.is_null() && (*xchp).ignore {
		xch = (*xchp).next;

		if xch.is_null() || (*xch).i1 - ((*xchp).i1 + (*xchp).chg1) >= max_ignorable {
			*xscr = xch;
		}
		
		xchp = (*xchp).next;
	}

	if (*xscr).is_null() {
		return std::ptr::null_mut();
	}

	let mut lxch = *xscr;
	xchp = *xscr;
	xch = (*xchp).next;
	while !xch.is_null() {
		let distance = (*xch).i1 - ((*xchp).i1 + (*xchp).chg1);
		if distance > max_common {
			break;
		}

		if distance < max_ignorable && (!(*xch).ignore || lxch == xchp) {
			lxch = xch;
			ignored = 0;
		} else if distance < max_ignorable && (*xch).ignore {
			ignored += (*xch).chg2;
		} else if lxch != xchp && (*xch).i1 + ignored - ((*lxch).i1 + (*lxch).chg1) > max_common {
			break;
		} else if !(*xch).ignore {
			lxch = xch;
			ignored = 0;
		} else {
			ignored += (*xch).chg2;
		}
		
		xchp = xch;
		xch = (*xch).next;
	}

	lxch
}


// #[no_mangle]
// pub(crate) unsafe extern "C" fn xdl_emit_diff(pair: *mut xdpair, xscr: *mut xdchange, ecb: *mut xdemitcb,
// 		  xecfg: *const xdemitconf) -> i32 {
// 	todo!()
// }
