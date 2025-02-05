use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3430428: FileFormat = FileFormat {
    id: 3_430_428,
    source_type: SourceType::Wikidata,
    name: "Rich Text Format Directory",
    extensions: &["rtfd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
