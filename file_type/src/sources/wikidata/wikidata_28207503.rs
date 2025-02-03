use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207503: FileFormat = FileFormat {
    id: 28_207_503,
    source_type: SourceType::Wikidata,
    name: "WinMiPS",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
