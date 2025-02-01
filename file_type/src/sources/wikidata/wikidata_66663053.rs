use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66663053: FileFormat = FileFormat {
    id: 66_663_053,
    puid: "wikidata/66663053",
    name: "eSuite Presentations Graphics",
    extensions: &["pg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
