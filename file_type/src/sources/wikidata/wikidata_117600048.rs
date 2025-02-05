use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117600048: FileFormat = FileFormat {
    id: 117_600_048,
    source_type: SourceType::Wikidata,
    name: "Digital Negative, version 1.5",
    extensions: &["dng"],
    media_types: &["image/tiff"],
    signatures: &[],
    related_formats: &[],
};
