infer_schema!("dotenv:DATABASE_URL");

table! {
    vidfull {
        id -> Integer,
        name -> VarChar,
        len -> BigInt,
        duration -> Float,
        bit_rate -> Integer,
        width -> Nullable<SmallInt>,
        height -> Nullable<SmallInt>,
        access -> Nullable<Timestamp>,
        modify -> Nullable<Timestamp>,
        viewed -> Nullable<Integer>,
        rate -> Nullable<SmallInt>,
        quality -> Nullable<SmallInt>,
        accumtime -> Nullable<Integer>,
        fpath -> Nullable<VarChar>,
        hash -> Nullable<VarChar>,
    }
}
