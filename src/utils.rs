use pgx::{pg_sys, tupdesc_get_typmod, tupdesc_get_typoid, PgBox, PgTupleDesc};

pub fn lookup_zdb_index_tupdesc(indexrel: &PgBox<pg_sys::RelationData>) -> PgTupleDesc {
    let tupdesc = PgBox::from_pg(indexrel.rd_att);
    // lookup the tuple descriptor for the rowtype we're *indexing*, rather than
    // using the tuple descriptor for the index definition itself
    PgTupleDesc::from_pg(unsafe {
        pg_sys::lookup_rowtype_tupdesc(
            tupdesc_get_typoid(&tupdesc, 1),
            tupdesc_get_typmod(&tupdesc, 1),
        )
    })
}
