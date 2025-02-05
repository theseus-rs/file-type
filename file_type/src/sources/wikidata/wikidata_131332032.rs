use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131332032: FileFormat = FileFormat {
    id: 131_332_032,
    source_type: SourceType::Wikidata,
    name: "TypoScript code",
    extensions: &["typoscript"],
    media_types: &["text/x-typoscript"],
    signatures: &[],
    related_formats: &[],
};
