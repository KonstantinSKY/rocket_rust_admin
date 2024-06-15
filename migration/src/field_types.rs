use sea_orm_migration::prelude::*;


pub fn id <T: Iden + 'static>(field_enum: T) -> ColumnDef {
    // Create and return a new ColumnDef instance in one line.
    ColumnDef::new(field_enum)
        .integer()
        .not_null()
        .auto_increment()
        .primary_key()
    .clone()
}

pub fn name <T: Iden + 'static>(field_enum: T) -> ColumnDef {
    ColumnDef::new(field_enum)
        .string()
        .not_null()
        .unique_key()
    .clone()
}

pub fn email <T: Iden + 'static>(field_enum: T) -> ColumnDef {
    ColumnDef::new(field_enum)
        .string()
        .not_null()
        .unique_key()
    .clone()
}

pub fn current_timestamp <T: Iden + 'static>(field_enum: T) -> ColumnDef {
    ColumnDef::new(field_enum)
        .timestamp()
        .not_null()
        .default(Expr::current_timestamp())
    .clone()
}