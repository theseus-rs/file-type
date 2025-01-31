use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122169650: FileFormat = FileFormat {
    id: 122_169_650,
    puid: "wikidata/122169650",
    name: "Password Cache File",
    extensions: &["epc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
