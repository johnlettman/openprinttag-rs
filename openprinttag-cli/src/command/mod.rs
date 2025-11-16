pub mod schema;

#[macro_export]
macro_rules! export { ( $( $m:tt )+ ) => { $(mod $m; pub use $m::*;)* }; }

#[macro_export]
macro_rules! register {
    (
        $(
            $( #[$meta:meta] )*
            $variant:ident ( $ty:path )
        )+
    ) => {
        #[derive(Debug, Clone, clap::Subcommand)]
        pub enum Commands {
            $(
                $( #[$meta] )*
                $variant($ty),
            )+
        }

        impl std::ops::Deref for Commands {
            type Target = dyn crate::command::Command;

            fn deref(&self) -> &Self::Target {
                match self {
                    $(
                        Commands::$variant(inner)
                            => register!(@maybe_deref inner; $( #[$meta] )* ),
                    )+
                }
            }
        }

        impl crate::command::Command for Commands {
            fn run(&self) -> crate::Result<()> {
                match self {
                    $(
                        Commands::$variant(inner)
                            => inner.run(),
                    )+
                }
            }
        }
    };

    // --- Helper: if the variant has #[subgroup], do NOT deref into it ---
    (@maybe_deref $inner:ident; #[subgroup] $($rest:tt)* ) => {
        compile_error!("Cannot deref a subgroup subcommand into `dyn Command`")
    };

    // --- Helper: otherwise, return &inner (requires it to implement Command) ---
    (@maybe_deref $inner:ident; $($rest:tt)* ) => {
        $inner
    };
}

export! {
    gen
}

register! {
    Gen(Gen)

    #[clap(subcommand)]
    Schema(schema::Commands)
}

pub trait Command {
    fn run(&self) -> crate::Result<()>;
}
