use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1570391: FileFormat = FileFormat {
    id: 1_570_391,
    puid: "wikidata/1570391",
    name: "Uuencoding",
    extensions: &["uu", "uue"],
    media_types: &["text/x-uuencode", "text/x-uuencode"],
    internal_signatures: &[],
    related_formats: &[],
};
