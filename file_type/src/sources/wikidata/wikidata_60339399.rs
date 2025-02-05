use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60339399: FileFormat = FileFormat {
    id: 60_339_399,
    source_type: SourceType::Wikidata,
    name: "Open Project File",
    extensions: &["pod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
