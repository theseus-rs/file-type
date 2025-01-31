use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79239537: FileFormat = FileFormat {
    id: 79_239_537,
    puid: "wikidata/79239537",
    name: "AOL Address Book",
    extensions: &["aby"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
