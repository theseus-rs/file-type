use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757779: FileFormat = FileFormat {
    id: 28_757_779,
    source_type: SourceType::Wikidata,
    name: "GME",
    extensions: &["gme"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
