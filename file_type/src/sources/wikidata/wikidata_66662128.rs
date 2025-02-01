use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66662128: FileFormat = FileFormat {
    id: 66_662_128,
    puid: "wikidata/66662128",
    name: "Lotus Organizer",
    extensions: &["org"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
