use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4037242: FileFormat = FileFormat {
    id: 4_037_242,
    puid: "wikidata/4037242",
    name: "Desktop.ini",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
