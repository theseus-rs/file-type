use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967486: FileFormat = FileFormat {
    id: 27_967_486,
    puid: "wikidata/27967486",
    name: "F4V",
    extensions: &["f4a", "f4b", "f4p", "f4v"],
    media_types: &["video/mp4", "video/mp4", "video/mp4", "video/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
