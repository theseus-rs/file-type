use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855846: FileFormat = FileFormat {
    id: 105_855_846,
    puid: "wikidata/105855846",
    name: "DocBook document (generic)",
    extensions: &["dbk", "xml"],
    media_types: &["application/docbook+xml", "application/docbook+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
