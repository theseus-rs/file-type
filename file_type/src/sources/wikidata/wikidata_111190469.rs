use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111190469: FileFormat = FileFormat {
    id: 111_190_469,
    source_type: SourceType::Wikidata,
    name: "Adobe Extension Data Markup Language Document",
    extensions: &["edml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
