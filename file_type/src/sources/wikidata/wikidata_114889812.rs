use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114889812: FileFormat = FileFormat {
    id: 114_889_812,
    puid: "wikidata/114889812",
    name: "Scrapbook Factory Deluxe Caledar file",
    extensions: &["scl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
