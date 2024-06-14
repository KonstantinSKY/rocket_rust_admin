use sea_orm_migration::prelude::*;


pub fn id<T: Iden + 'static>(field_enum: T) -> ColumnDef {
    // Create and return a new ColumnDef instance in one line.
    // This assumes that each chained method returns the modified ColumnDef as `&mut ColumnDef`.
    let mut column_def = ColumnDef::new(field_enum);
    column_def.integer()
              .not_null()
              .auto_increment()
              .primary_key();

    column_def  // Return the modified ColumnDef
}

