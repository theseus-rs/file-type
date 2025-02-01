use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117192588: FileFormat = FileFormat {
    id: 117_192_588,
    puid: "wikidata/117192588",
    name: "Photoshop PDF",
    extensions: &["pdf", "pdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
