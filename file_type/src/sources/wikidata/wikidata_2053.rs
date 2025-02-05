use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2053: FileFormat = FileFormat {
    id: 2_053,
    source_type: SourceType::Wikidata,
    name: "HTML5",
    extensions: &["htm", "html"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
