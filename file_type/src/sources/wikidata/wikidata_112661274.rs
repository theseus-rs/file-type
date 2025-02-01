use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661274: FileFormat = FileFormat {
    id: 112_661_274,
    puid: "wikidata/112661274",
    name: "Lightscape Solution file",
    extensions: &["ls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
