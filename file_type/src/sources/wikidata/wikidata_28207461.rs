use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207461: FileFormat = FileFormat {
    id: 28_207_461,
    source_type: SourceType::Wikidata,
    name: "VITec Image Format",
    extensions: &["vit", "vitec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
