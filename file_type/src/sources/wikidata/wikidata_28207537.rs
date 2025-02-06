use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207537: FileFormat = FileFormat {
    id: 28_207_537,
    source_type: SourceType::Wikidata,
    name: "Xerox Doodle brush",
    extensions: &["brush"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
