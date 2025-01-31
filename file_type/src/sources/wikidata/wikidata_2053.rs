use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2053: FileFormat = FileFormat {
    id: 2_053,
    puid: "wikidata/2053",
    name: "HTML5",
    extensions: &["htm", "html"],
    media_types: &["text/html", "text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
