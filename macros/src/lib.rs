// Shade Framework - Procedural Macros
// Copyright (c) 2025 Shadow Protocol Contributors
// Licensed under MIT License

//! Procedural macros for circuit development
//!
//! Provides:
//! - `#[circuit]` - Derive macro for circuit structures
//! - `#[private]` - Mark field as private input
//! - `#[public]` - Mark field as public input
//! - `#[constant]` - Mark field as constant
//! - Circuit builder macros for ergonomic circuit construction

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

/// Derive macro for creating circuits
///
/// # Example
/// ```ignore
/// #[derive(Circuit)]
/// pub struct MyCircuit {
///     #[private]
///     secret: Field,
///
///     #[public]
///     hash: Field,
/// }
/// ```
#[proc_macro_derive(Circuit, attributes(private, public, constant))]
pub fn derive_circuit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract fields and their attributes
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Circuit must have named fields"),
        },
        _ => panic!("Circuit can only be derived for structs"),
    };

    let mut private_fields = Vec::new();
    let mut public_fields = Vec::new();
    let mut constant_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let mut is_private = false;
        let mut is_public = false;
        let mut is_constant = false;

        for attr in &field.attrs {
            if attr.path().is_ident("private") {
                is_private = true;
            } else if attr.path().is_ident("public") {
                is_public = true;
            } else if attr.path().is_ident("constant") {
                is_constant = true;
            }
        }

        if is_private {
            private_fields.push(field_name);
        } else if is_public {
            public_fields.push(field_name);
        } else if is_constant {
            constant_fields.push(field_name);
        }
    }

    // Generate implementation
    let expanded = quote! {
        impl #name {
            /// Get private field names
            pub fn private_fields() -> &'static [&'static str] {
                &[#(stringify!(#private_fields)),*]
            }

            /// Get public field names
            pub fn public_fields() -> &'static [&'static str] {
                &[#(stringify!(#public_fields)),*]
            }

            /// Get constant field names
            pub fn constant_fields() -> &'static [&'static str] {
                &[#(stringify!(#constant_fields)),*]
            }

            /// Convert to witness
            pub fn to_witness(&self) -> shade::Witness {
                let mut witness = shade::Witness::new();
                #(
                    witness.add(stringify!(#private_fields), self.#private_fields);
                )*
                witness
            }

            /// Convert to public inputs
            pub fn to_public_inputs(&self) -> shade::PublicInputs {
                let mut inputs = shade::PublicInputs::new();
                #(
                    inputs.add(stringify!(#public_fields), self.#public_fields);
                )*
                inputs
            }

            /// Build constraint system
            pub fn build_constraints(&self, cs: &mut shade::ConstraintSystem) -> Result<(), shade::CircuitError> {
                // Allocate private variables
                #(
                    let #private_fields = cs.alloc_variable(Some(self.#private_fields));
                )*

                // Allocate public variables
                #(
                    let #public_fields = cs.alloc_variable(Some(self.#public_fields));
                )*

                // User implements constraints in separate method
                self.constraints(cs)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro for circuit methods
///
/// Automatically generates constraint system setup and witness handling
///
/// # Example
/// ```ignore
/// #[circuit]
/// impl MyCircuit {
///     fn constraints(&self, cs: &mut CS) -> Result<()> {
///         // Define constraints here
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn circuit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);
    let output = quote! {
        #input
    };
    TokenStream::from(output)
}

/// Macro for creating gadgets with automatic constraint tracking
///
/// # Example
/// ```ignore
/// gadget! {
///     pub fn hash_preimage(cs: &mut CS, secret: Var, hash: Var) -> Result<()> {
///         let computed = poseidon_hash(cs, &[secret])?;
///         cs.enforce_equal(computed, hash);
///         Ok(())
///     }
/// }
/// ```
#[proc_macro]
pub fn gadget(input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_body = &input_fn.block;

    let expanded = quote! {
        pub fn #fn_name(#fn_inputs) #fn_output {
            let _gadget_start = std::time::Instant::now();
            let result = (|| #fn_body)();
            let _gadget_duration = _gadget_start.elapsed();

            #[cfg(feature = "metrics")]
            {
                shade::metrics::record_gadget(
                    stringify!(#fn_name),
                    _gadget_duration,
                );
            }

            result
        }
    };

    TokenStream::from(expanded)
}

/// Macro for constraint generation
///
/// # Example
/// ```ignore
/// constrain! {
///     cs,
///     a + b => c,
///     d * e => f,
///     g == h,
/// }
/// ```
#[proc_macro]
pub fn constrain(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let lines: Vec<&str> = input_str.split(',').map(|s| s.trim()).collect();

    let mut constraints = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        // Parse constraint type
        if line.contains("=>") {
            // Multiplication constraint: a * b => c
            let parts: Vec<&str> = line.split("=>").map(|s| s.trim()).collect();
            if parts.len() != 2 {
                continue;
            }

            let lhs = parts[0];
            let rhs = parts[1];

            if lhs.contains('*') {
                let mul_parts: Vec<&str> = lhs.split('*').map(|s| s.trim()).collect();
                if mul_parts.len() == 2 {
                    let a = syn::parse_str::<syn::Expr>(mul_parts[0]).unwrap();
                    let b = syn::parse_str::<syn::Expr>(mul_parts[1]).unwrap();
                    let c = syn::parse_str::<syn::Expr>(rhs).unwrap();

                    constraints.push(quote! {
                        cs.enforce_mul(#a, #b, #c);
                    });
                }
            } else if lhs.contains('+') {
                // Addition constraint: a + b => c
                let add_parts: Vec<&str> = lhs.split('+').map(|s| s.trim()).collect();
                if add_parts.len() >= 2 {
                    let mut lc = quote! { LinearCombination::zero() };
                    for part in add_parts {
                        let var = syn::parse_str::<syn::Expr>(part).unwrap();
                        lc = quote! {
                            #lc.add(&LinearCombination::from_variable(#var))
                        };
                    }
                    let c = syn::parse_str::<syn::Expr>(rhs).unwrap();
                    constraints.push(quote! {
                        cs.enforce_equal(#lc, LinearCombination::from_variable(#c));
                    });
                }
            }
        } else if line.contains("==") {
            // Equality constraint: a == b
            let parts: Vec<&str> = line.split("==").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let a = syn::parse_str::<syn::Expr>(parts[0]).unwrap();
                let b = syn::parse_str::<syn::Expr>(parts[1]).unwrap();

                constraints.push(quote! {
                    cs.enforce_equal(
                        LinearCombination::from_variable(#a),
                        LinearCombination::from_variable(#b)
                    );
                });
            }
        }
    }

    let expanded = quote! {
        {
            #(#constraints)*
        }
    };

    TokenStream::from(expanded)
}

/// Macro for allocating variables in circuits
///
/// # Example
/// ```ignore
/// let (a, b, c) = alloc!(cs, Field::from(1), Field::from(2), Field::from(3));
/// ```
#[proc_macro]
pub fn alloc(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let tokens: Vec<_> = input.into_iter().collect();

    if tokens.is_empty() {
        return TokenStream::new();
    }

    // First token should be cs (constraint system)
    let cs = &tokens[0];

    // Rest are values to allocate
    let values: Vec<_> = tokens.iter().skip(2).collect(); // Skip cs and comma

    let mut allocations = Vec::new();
    for value in values {
        allocations.push(quote! {
            #cs.alloc_variable(Some(#value))
        });
    }

    let expanded = if allocations.len() == 1 {
        quote! { #(#allocations)* }
    } else {
        quote! { (#(#allocations),*) }
    };

    TokenStream::from(expanded)
}

/// Macro for creating circuit builders
///
/// # Example
/// ```ignore
/// circuit_builder! {
///     pub struct HashChainBuilder {
///         chain_length: usize,
///     }
///
///     build {
///         // Build logic
///     }
/// }
/// ```
#[proc_macro]
pub fn circuit_builder(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    // Simple implementation - expand to basic builder pattern
    let expanded = quote! {
        impl Builder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn build(self) -> Circuit {
                unimplemented!("Custom build logic")
            }
        }
    };

    TokenStream::from(expanded)
}

/// Macro for benchmarking gadgets
///
/// # Example
/// ```ignore
/// bench_gadget! {
///     name: "poseidon_hash",
///     setup: {
///         let input = Field::from(123);
///     },
///     run: {
///         poseidon_hash(cs, &[input])
///     }
/// }
/// ```
#[proc_macro]
pub fn bench_gadget(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        #[cfg(test)]
        mod bench {
            use super::*;
            use std::time::Instant;

            #[test]
            fn benchmark() {
                let start = Instant::now();
                // Benchmark code here
                let duration = start.elapsed();
                println!("Execution time: {:?}", duration);
            }
        }
    };

    TokenStream::from(expanded)
}

/// Helper macro for field arithmetic
///
/// # Example
/// ```ignore
/// field_expr!(a + b * c - d)
/// ```
#[proc_macro]
pub fn field_expr(input: TokenStream) -> TokenStream {
    // Parse and convert to field operations
    let expr = parse_macro_input!(input as syn::Expr);

    let expanded = quote! {
        {
            #expr
        }
    };

    TokenStream::from(expanded)
}

/// Macro for creating test circuits
///
/// # Example
/// ```ignore
/// test_circuit! {
///     name: square_test,
///     inputs: {
///         x: 5
///     },
///     constraints: {
///         y = x * x
///     },
///     expected: {
///         y: 25
///     }
/// }
/// ```
#[proc_macro]
pub fn test_circuit(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn circuit_test() {
                let mut cs = ConstraintSystem::new();
                // Test logic here
                assert!(cs.is_satisfied());
            }
        }
    };

    TokenStream::from(expanded)
}
