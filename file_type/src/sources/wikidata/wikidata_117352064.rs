use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117352064: FileFormat = FileFormat {
    id: 117_352_064,
    source_type: SourceType::Wikidata,
    name: "Capture Design",
    extensions: &["dsn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
