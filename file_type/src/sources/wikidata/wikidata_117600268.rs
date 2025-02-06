use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117600268: FileFormat = FileFormat {
    id: 117_600_268,
    source_type: SourceType::Wikidata,
    name: "Digital Negative, version 1.6",
    extensions: &["dng", "tif"],
    media_types: &["image/tiff"],
    signatures: &[],
    related_formats: &[],
};
