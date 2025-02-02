use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123541561: FileFormat = FileFormat {
    id: 123_541_561,
    source_type: SourceType::Wikidata,
    name: "MPEG-4 playlist",
    extensions: &["m4u", "mxu"],
    media_types: &["video/vnd.mpegurl"],
    internal_signatures: &[],
    related_formats: &[],
};
