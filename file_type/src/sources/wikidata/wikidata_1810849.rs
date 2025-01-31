use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1810849: FileFormat = FileFormat {
    id: 1_810_849,
    puid: "wikidata/1810849",
    name: "XLIFF",
    extensions: &["xlf"],
    media_types: &["application/xliff+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
