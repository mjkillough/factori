/// Helper for flattening repition in proc-macro-rules.
macro_rules! for_repition {
    ( ( $($name:ident),* ) => $body:tt) => {{
        use itertools::izip;

        #[allow(unused_parens)]
        for ( $($name),* ) in izip!( $($name),* ) {
            $body
        }
    }};
}
