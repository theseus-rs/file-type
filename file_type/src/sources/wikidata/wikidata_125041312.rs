use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125041312: FileFormat = FileFormat {
    id: 125_041_312,
    source_type: SourceType::Wikidata,
    name: "ZynAddSubFX Presets File",
    extensions: &["xpz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
