use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661280: FileFormat = FileFormat {
    id: 112_661_280,
    puid: "wikidata/112661280",
    name: "Lightscape View file",
    extensions: &["vw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
