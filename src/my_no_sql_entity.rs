pub struct TableInfo {
    pub table_name: &'static str,
}

pub trait MyNoSqlEntity {
    fn get_partition_key(&self) -> &str;
    fn get_row_key(&self) -> &str;
    fn get_time_stamp(&self) -> i64;

    fn get_table_info() -> TableInfo;
}
