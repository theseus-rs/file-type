use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99184084: FileFormat = FileFormat {
    id: 99_184_084,
    puid: "wikidata/99184084",
    name: "Atom web feed",
    extensions: &["atom", "xml"],
    media_types: &["application/atom+xml", "application/atom+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
