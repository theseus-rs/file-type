use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48809727: FileFormat = FileFormat {
    id: 48_809_727,
    source_type: SourceType::Wikidata,
    name: "dBASE Text Memo",
    extensions: &["dbt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
