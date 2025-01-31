use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124622467: FileFormat = FileFormat {
    id: 124_622_467,
    puid: "wikidata/124622467",
    name: "TEI/XML",
    extensions: &["odd", "xml"],
    media_types: &["application/tei+xml", "application/tei+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
