pub trait DatabaseMeta  {
    type Schema: SchemaMeta;
    fn get_name() -> String;
    fn get_schema(name: String) -> Option<Box<Self::Schema>>;
    fn get_schemas() -> Vec<Box<Self::Schema>>;
    fn list_schemas() -> Vec<String>;
}

pub trait SchemaMeta {
    type Table: TableMeta;

    fn get_name() -> String;
    fn get_table(name: String) -> Option<Box<Self::Table>>;
    fn get_tables() -> Vec<Box<Self::Table>>;
    fn list_tables() -> Vec<String>;
}

pub trait TableMeta {
    type Column: ColumnMeta;
    type Constraint: ConstraintMeta;
    type Index: IndexMeta;
    fn get_name() -> String;
    fn get_column(name: String) -> Option<Box<Self::Column>>;
    fn get_columns() -> Vec<Box<Self::Column>>;
    fn list_columns(name: String) -> Vec<String>;
    fn get_constraint(name: String) -> Option<Self::Constraint>;
    fn get_constraints(name: String) -> Vec<Self::Constraint>;
    fn list_constraints(name: String) -> Vec<String>;
    fn get_index(name: String) -> Option<Self::Index>;
    fn get_indexes(name: String) -> Vec<Self::Index>;
    fn list_index(name: String) -> Vec<String>;
    fn trigger(name: String) -> Vec<String>;

}

pub trait ColumnMeta {
    type Type: TypeMeta;
    fn get_name() -> String;
    fn get_type() -> Box<Self::Type>;
    fn get_default() -> Box<ValueMeta>;
}

pub trait TypeMeta {
    fn get_identifier() -> String;
}

pub trait DataTypeMeta{

}

pub trait ValueMeta {

}

pub trait ConstraintMeta {

}

pub trait IndexMeta {

}
