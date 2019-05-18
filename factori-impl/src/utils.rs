/// Helper for flattening repitition in proc-macro-rules.
macro_rules! for_repitition {
    ( ( $($name:ident),* ) => $body:tt) => {{
        use itertools::izip;

        #[allow(unused_parens)]
        for ( $($name),* ) in izip!( $($name),* ) {
            $body
        }
    }};
}
