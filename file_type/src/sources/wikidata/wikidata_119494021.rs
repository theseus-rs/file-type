use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119494021: FileFormat = FileFormat {
    id: 119_494_021,
    source_type: SourceType::Wikidata,
    name: "SnagIt Capture File",
    extensions: &["snag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
