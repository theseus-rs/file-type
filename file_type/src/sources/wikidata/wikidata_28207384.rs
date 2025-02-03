use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207384: FileFormat = FileFormat {
    id: 28_207_384,
    source_type: SourceType::Wikidata,
    name: "TIFF/IT",
    extensions: &["tif", "tiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
