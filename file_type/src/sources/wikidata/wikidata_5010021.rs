use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5010021: FileFormat = FileFormat {
    id: 5_010_021,
    puid: "wikidata/5010021",
    name: "CDX Format",
    extensions: &["cdx"],
    media_types: &["chemical/x-cdx"],
    internal_signatures: &[],
    related_formats: &[],
};
