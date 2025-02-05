use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7670377: FileFormat = FileFormat {
    id: 7_670_377,
    source_type: SourceType::Wikidata,
    name: "Tagged Image File Format/Electronic Photography",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff"],
    signatures: &[],
    related_formats: &[],
};
