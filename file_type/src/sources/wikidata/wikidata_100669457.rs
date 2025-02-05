use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100669457: FileFormat = FileFormat {
    id: 100_669_457,
    source_type: SourceType::Wikidata,
    name: "Apple iWork Document, version 14",
    extensions: &["iwa", "key", "numbers", "pages", "template"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
