use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856033: FileFormat = FileFormat {
    id: 105_856_033,
    source_type: SourceType::Wikidata,
    name: "SPECCTRA Design",
    extensions: &["dsn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
