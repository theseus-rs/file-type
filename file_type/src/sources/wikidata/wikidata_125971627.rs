use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125971627: FileFormat = FileFormat {
    id: 125_971_627,
    puid: "wikidata/125971627",
    name: "gemtext",
    extensions: &["gmi"],
    media_types: &["text/gemini"],
    internal_signatures: &[],
    related_formats: &[],
};
