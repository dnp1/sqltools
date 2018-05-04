struct DataType {
    name: String,
    scalar: bool,
}

enum DataT<D, U> {
    Standard(D),
    UserDefined(U),
}

struct ColumnConstraint;

struct Column {
    name: String,
    data_type: DataType,
    constraints: Vec<ColumnConstraint>,
}

struct TableConstraints;

struct Table {
    columns: Vec<Column>,
    constraints: Vec<ColumnConstraint>,
}

struct Domain;

struct Schema {
    domains: Vec<Domain>,
    tables: Vec<Table>,
}
