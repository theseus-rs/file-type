use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62626012: FileFormat = FileFormat {
    id: 62_626_012,
    source_type: SourceType::Wikidata,
    name: "HyperText Markup Language",
    extensions: &["htm", "html", "xht", "xhtml"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
