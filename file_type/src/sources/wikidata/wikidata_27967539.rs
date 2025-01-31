use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967539: FileFormat = FileFormat {
    id: 27_967_539,
    puid: "wikidata/27967539",
    name: "Advanced Video Coding",
    extensions: &["mp4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
