#![allow(non_camel_case_types)]

use libc;
use std::ops::Deref;

use {ViewModel, ViewModelHandle, ViewModelObserver};

/**
 * rust_byte_slice is a pointer-to-bytes with an included length.
 */
#[repr(C)]
pub struct rust_byte_slice {
    pub bytes: *const u8,
    pub len: libc::size_t,
}

/**
 * `view_model`s are handed out by a `view_model_handle`. See the `view_model_*` functions
 * below for usage.
 */
#[repr(C)]
pub struct view_model(ViewModel);

// Avoid having to say "vm.0" in the functions that deal with view_models.
impl Deref for view_model {
    type Target = ViewModel;
    fn deref(&self) -> &ViewModel {
        &self.0
    }
}

/**
 * `view_model_handle` is managed by Rust and vends updated `view_model`s.
 */
#[repr(C)]
pub struct view_model_handle(ViewModelHandle);

/**
 * A `view_model_observer` is attached to a `view_model_handle` and is the mechanism by
 * which Rust will notify interested clients about changes to the `view_model`. The functions
 * provided in a `view_model_observer` will be called from different threads but never at the same
 * time.
 */
#[repr(C)]
pub struct view_model_observer {
    /// User-specific data.
    pub user: *mut libc::c_void,

    /// Hook to perform any cleanup needed on `user`. Will be called exactly once when the
    /// `view_model_handle` that owns this observer is deallocated.
    pub destroy_user: extern "C" fn(user: *mut libc::c_void),

    /// Callback function executed whenever a new item is inserted into the view model
    /// (at the given index). The observer is responsible for calling `view_model_destroy`
    /// on `view_model` when they are finished with it.
    pub inserted_item: extern "C" fn(user: *mut libc::c_void,
                                         view_model: *mut view_model,
                                         index: libc::size_t)
                                        ,
    /// Callback function executed whenever a new item is removed from the view model
    /// (at the given index). The observer is responsible for calling `view_model_destroy`
    /// on `view_model` when they are finished with it.
    pub removed_item: extern "C" fn(user: *mut libc::c_void,
                                        view_model: *mut view_model,
                                        index: libc::size_t)
                                       ,
    /// Callback function executed whenever an existing item in the view model is modified
    /// (at the given index). The observer is responsible for calling `view_model_destroy`
    /// on `view_model` when they are finished with it.
    pub modified_item: extern "C" fn(user: *mut libc::c_void,
                                         view_model: *mut view_model,
                                         index: libc::size_t)
                                        ,
}

unsafe impl Send for view_model_observer {}

/**
 * Create a new `view_model_handle` with the given number of worker threads and the given
 * `view_model_observer`. The initial view model state will be stored into `*out_view_model`.
 * The caller must call `view_model_handle_destroy` on the returned handle to avoid
 * leaking resources, and must call `view_model_destroy` on the `view_model` returned in
 * `out_view_model` to avoid leaking resources.
 */
#[no_mangle]
pub unsafe extern "C" fn view_model_handle_new(num_threads: libc::size_t,
                                               observer: view_model_observer,
                                               out_view_model: *mut *mut view_model)
                                               -> *mut view_model_handle {
    let observer = ViewModelObserverWrapper(observer);
    let (handle, vm) = ViewModelHandle::new(num_threads, observer);
    *out_view_model = Box::into_raw(Box::new(view_model(vm)));
    Box::into_raw(Box::new(view_model_handle(handle)))
}

/**
 * Free any resources and memory associated with a `view_model_handle`.
 */
#[no_mangle]
pub unsafe extern "C" fn view_model_handle_destroy(handle: *mut view_model_handle) {
    let _ = Box::from_raw(handle);
}

/**
 * Free any resources and memory associated with a `view_model`.
 */
#[no_mangle]
pub unsafe extern "C" fn view_model_destroy(vm: *mut view_model) {
    let _ = Box::from_raw(vm);
}

/**
 * Get a count of the number of string values stored in `vm`.
 */
#[no_mangle]
pub unsafe extern "C" fn view_model_len(vm: *mut view_model) -> libc::size_t {
    let vm = &*vm;
    vm.len() as libc::size_t
}

/**
 * Get the string at index `index`. `index` must be between 0 and `view_model_len(vm) - 1`,
 * inclusive.
 *
 * The pointer contained in the returned byte slice is valid until `vm` is destroyed.
 */
#[no_mangle]
pub unsafe extern "C" fn view_model_value_at_index(vm: *mut view_model,
                                                   index: libc::size_t)
                                                   -> rust_byte_slice {
    let vm = &*vm;
    let s = vm.value_at_index(index as usize);
    rust_byte_slice {
        bytes: s.as_ptr(),
        len: s.len() as libc::size_t,
    }
}

struct ViewModelObserverWrapper(view_model_observer);

// Avoid having to say "self.0" in all our ViewModelObserverWrapper functions below.
impl Deref for ViewModelObserverWrapper {
    type Target = view_model_observer;
    fn deref(&self) -> &view_model_observer {
        &self.0
    }
}

impl Drop for ViewModelObserverWrapper {
    fn drop(&mut self) {
        (self.destroy_user)(self.user);
    }
}

impl ViewModelObserver for ViewModelObserverWrapper {
    fn inserted_item(&self, vm: ViewModel, index: usize) {
        let vm = Box::into_raw(Box::new(view_model(vm)));
        (self.inserted_item)(self.user, vm, index as libc::size_t);
    }
    fn removed_item(&self, vm: ViewModel, index: usize) {
        let vm = Box::into_raw(Box::new(view_model(vm)));
        (self.removed_item)(self.user, vm, index as libc::size_t);
    }
    fn modified_item(&self, vm: ViewModel, index: usize) {
        let vm = Box::into_raw(Box::new(view_model(vm)));
        (self.modified_item)(self.user, vm, index as libc::size_t);
    }
}
