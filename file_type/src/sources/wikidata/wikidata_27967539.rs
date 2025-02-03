use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967539: FileFormat = FileFormat {
    id: 27_967_539,
    source_type: SourceType::Wikidata,
    name: "Advanced Video Coding",
    extensions: &["mp4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
