use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66685987: FileFormat = FileFormat {
    id: 66_685_987,
    puid: "wikidata/66685987",
    name: "OR4",
    extensions: &["or4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
