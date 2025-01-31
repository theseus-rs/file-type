use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48809727: FileFormat = FileFormat {
    id: 48_809_727,
    puid: "wikidata/48809727",
    name: "dBASE Text Memo",
    extensions: &["dbt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
