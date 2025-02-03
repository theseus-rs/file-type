use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117600048: FileFormat = FileFormat {
    id: 117_600_048,
    source_type: SourceType::Wikidata,
    name: "Digital Negative, version 1.5",
    extensions: &["dng"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};
