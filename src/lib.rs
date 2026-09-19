#![feature(generic_const_exprs)]
//#![feature(min_adt_const_params)]
//#![feature(min_generic_const_args)]

//#![feature(const_index)]
//#![feature(const_array)]
//#![feature(generic_const_args)]
//#![feature(generic_const_items)]
//#![feature(generic_const_parameter_types)]
//#![feature(impl_trait_in_bindings)]



// old and therefore no longer pub
mod _vectors_first_version;
mod _matrix_first_version;
mod _matrix_second_version;

mod tests;

//pub mod statistics;
pub mod traits;
pub mod enums;
pub mod general_math;
pub mod geometry_3d;
pub mod macros;
//pub mod currently_unused___matrices;

//pub mod functions_and_math;


pub mod chem_matrix_stuff;
pub mod triangulation;
pub mod rotation_stuff;
//pub mod function_stuff;
pub mod matrix_fix_rotations;

pub mod matrices;
pub mod vectors;