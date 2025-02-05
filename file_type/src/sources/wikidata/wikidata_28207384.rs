use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207384: FileFormat = FileFormat {
    id: 28_207_384,
    source_type: SourceType::Wikidata,
    name: "TIFF/IT",
    extensions: &["tif", "tiff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
