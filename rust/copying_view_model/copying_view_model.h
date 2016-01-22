
#ifndef cheddar_generated_copying_view_model_h
#define cheddar_generated_copying_view_model_h


#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



/**
 * rust_byte_slice is a pointer-to-bytes with an included length.
 */
typedef struct rust_byte_slice {
	const uint8_t* bytes;
	size_t len;
} rust_byte_slice;

/**
 * `view_model`s are handed out by a `view_model_handle`. See the `view_model_*` functions
 * below for usage.
 */
typedef struct view_model view_model;

/**
 * `view_model_handle` is managed by Rust and vends updated `view_model`s.
 */
typedef struct view_model_handle view_model_handle;

/**
 * A `view_model_observer` is attached to a `view_model_handle` and is the mechanism by
 * which Rust will notify interested clients about changes to the `view_model`. The functions
 * provided in a `view_model_observer` will be called from different threads but never at the same
 * time.
 */
typedef struct view_model_observer {
	/// User-specific data.
	void* user;
	/// Hook to perform any cleanup needed on `user`. Will be called exactly once when the
	/// `view_model_handle` that owns this observer is deallocated.
	void (*destroy_user)(void* user);
	/// Callback function executed whenever a new item is inserted into the view model
	/// (at the given index). The observer is responsible for calling `view_model_destroy`
	/// on `view_model` when they are finished with it.
	void (*inserted_item)(void* user, view_model* view_model, size_t index);
	/// Callback function executed whenever a new item is removed from the view model
	/// (at the given index). The observer is responsible for calling `view_model_destroy`
	/// on `view_model` when they are finished with it.
	void (*removed_item)(void* user, view_model* view_model, size_t index);
	/// Callback function executed whenever an existing item in the view model is modified
	/// (at the given index). The observer is responsible for calling `view_model_destroy`
	/// on `view_model` when they are finished with it.
	void (*modified_item)(void* user, view_model* view_model, size_t index);
} view_model_observer;

/**
 * Create a new `view_model_handle` with the given number of worker threads and the given
 * `view_model_observer`. The initial view model state will be stored into `*out_view_model`.
 * The caller must call `view_model_handle_destroy` on the returned handle to avoid
 * leaking resources, and must call `view_model_destroy` on the `view_model` returned in
 * `out_view_model` to avoid leaking resources.
 */
view_model_handle* view_model_handle_new(size_t num_threads, view_model_observer observer, view_model** out_view_model);

/**
 * Free any resources and memory associated with a `view_model_handle`.
 */
void view_model_handle_destroy(view_model_handle* handle);

/**
 * Free any resources and memory associated with a `view_model`.
 */
void view_model_destroy(view_model* vm);

/**
 * Get a count of the number of string values stored in `vm`.
 */
size_t view_model_len(view_model* vm);

/**
 * Get the string at index `index`. `index` must be between 0 and `view_model_len(vm) - 1`,
 * inclusive.
 *
 * The pointer contained in the returned byte slice is valid until `vm` is destroyed.
 */
rust_byte_slice view_model_value_at_index(view_model* vm, size_t index);



#ifdef __cplusplus
}
#endif


#endif
