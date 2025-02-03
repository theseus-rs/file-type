use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600494: FileFormat = FileFormat {
    id: 28_600_494,
    source_type: SourceType::Wikidata,
    name: "Dev-Cpp project",
    extensions: &["dev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
