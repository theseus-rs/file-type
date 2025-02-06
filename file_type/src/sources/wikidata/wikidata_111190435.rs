use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111190435: FileFormat = FileFormat {
    id: 111_190_435,
    source_type: SourceType::Wikidata,
    name: "JavaServer Page Document",
    extensions: &["jst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
