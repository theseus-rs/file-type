use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852037: FileFormat = FileFormat {
    id: 105_852_037,
    puid: "wikidata/105852037",
    name: "Digital Micrograph Script",
    extensions: &["s"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
