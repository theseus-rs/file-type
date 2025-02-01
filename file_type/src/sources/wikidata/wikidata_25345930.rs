use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25345930: FileFormat = FileFormat {
    id: 25_345_930,
    puid: "wikidata/25345930",
    name: "Citrine",
    extensions: &["ctr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
