use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855935: FileFormat = FileFormat {
    id: 105_855_935,
    puid: "wikidata/105855935",
    name: "DocBook document (v4.x)",
    extensions: &["dbk", "xml"],
    media_types: &["application/docbook+xml", "application/docbook+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
