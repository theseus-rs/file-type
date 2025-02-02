use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125041312: FileFormat = FileFormat {
    id: 125_041_312,
    source_type: SourceType::Wikidata,
    name: "ZynAddSubFX Presets File",
    extensions: &["xpz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
