use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100297628: FileFormat = FileFormat {
    id: 100_297_628,
    source_type: SourceType::Wikidata,
    name: "Flow Charting file format, version 3",
    extensions: &["fcd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
