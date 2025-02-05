use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207346: FileFormat = FileFormat {
    id: 28_207_346,
    source_type: SourceType::Wikidata,
    name: "Image Capture Board",
    extensions: &["icb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
