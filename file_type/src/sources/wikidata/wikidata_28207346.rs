use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207346: FileFormat = FileFormat {
    id: 28_207_346,
    source_type: SourceType::Wikidata,
    name: "Image Capture Board",
    extensions: &["icb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
