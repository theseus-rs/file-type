use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207537: FileFormat = FileFormat {
    id: 28_207_537,
    source_type: SourceType::Wikidata,
    name: "Xerox Doodle brush",
    extensions: &["brush"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
