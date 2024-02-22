pub struct Production {
    pub symbol: String,
    pub next_production: String,
    pub concatted_production: String,
    pub next_production1: String, // used for cases like A -> CD | CF
    pub concatted_production1: String,
}

pub impl Production {
    fn goesToSymbol(sym: &String) -> bool {
        return next_production == sym
            || concatted_production1 == sym
            || next_production1 == sym
            || concatted_production1 == sym;
    }
}
