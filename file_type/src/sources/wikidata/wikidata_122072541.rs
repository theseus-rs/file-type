use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122072541: FileFormat = FileFormat {
    id: 122_072_541,
    puid: "wikidata/122072541",
    name: "Rhapsody File",
    extensions: &["rhp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
