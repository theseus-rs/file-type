use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7708383: FileFormat = FileFormat {
    id: 7_708_383,
    source_type: SourceType::Wikidata,
    name: "textClipping",
    extensions: &["textClipping"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
