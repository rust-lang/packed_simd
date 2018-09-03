//! Includes the ISPC implementations.
use *;

ispc_module!(aobench);

pub fn ao<S: Scene>(_scene: &mut S, nsubsamples: usize, img: &mut ::Image) {
    let (w, h) = img.size();
    unsafe {
        aobench::ao_ispc(
            w as i32,
            h as i32,
            nsubsamples as i32,
            img.fdata.as_mut_ptr(),
        )
    }
}

pub fn ao_tasks<S: Scene>(
    _scene: &mut S,
    nsubsamples: usize,
    img: &mut ::Image,
) {
    let (w, h) = img.size();
    unsafe {
        aobench::ao_ispc_tasks(
            w as i32,
            h as i32,
            nsubsamples as i32,
            img.fdata.as_mut_ptr(),
        )
    }
}
