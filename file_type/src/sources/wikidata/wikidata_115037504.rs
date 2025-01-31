use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115037504: FileFormat = FileFormat {
    id: 115_037_504,
    puid: "wikidata/115037504",
    name: "Extensible Markup Language 1.1",
    extensions: &["xml", "xml"],
    media_types: &["application/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
