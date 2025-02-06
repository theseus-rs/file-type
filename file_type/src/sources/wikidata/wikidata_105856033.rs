use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856033: FileFormat = FileFormat {
    id: 105_856_033,
    source_type: SourceType::Wikidata,
    name: "SPECCTRA Design",
    extensions: &["dsn"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
