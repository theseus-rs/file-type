use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51718267: FileFormat = FileFormat {
    id: 51_718_267,
    puid: "wikidata/51718267",
    name: "Schedule+ Contacts",
    extensions: &["scd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
