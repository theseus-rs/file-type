use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049476: FileFormat = FileFormat {
    id: 28_049_476,
    source_type: SourceType::Wikidata,
    name: "RGB Intermediate Format",
    extensions: &["rgb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
