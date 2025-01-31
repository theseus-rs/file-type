use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62626012: FileFormat = FileFormat {
    id: 62_626_012,
    puid: "wikidata/62626012",
    name: "HyperText Markup Language",
    extensions: &["htm", "html", "xht", "xhtml"],
    media_types: &["text/html", "text/html", "text/html", "text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
