use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117600268: FileFormat = FileFormat {
    id: 117_600_268,
    source_type: SourceType::Wikidata,
    name: "Digital Negative, version 1.6",
    extensions: &["dng", "tif"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};
