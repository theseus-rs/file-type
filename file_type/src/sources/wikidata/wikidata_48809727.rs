use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48809727: FileFormat = FileFormat {
    id: 48_809_727,
    source_type: SourceType::Wikidata,
    name: "dBASE Text Memo",
    extensions: &["dbt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
