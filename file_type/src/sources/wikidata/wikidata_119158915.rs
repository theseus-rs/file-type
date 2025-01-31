use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119158915: FileFormat = FileFormat {
    id: 119_158_915,
    puid: "wikidata/119158915",
    name: "Digital Image PNG Plus",
    extensions: &["png"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
