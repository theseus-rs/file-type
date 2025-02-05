use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207461: FileFormat = FileFormat {
    id: 28_207_461,
    source_type: SourceType::Wikidata,
    name: "VITec Image Format",
    extensions: &["vit", "vitec"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
