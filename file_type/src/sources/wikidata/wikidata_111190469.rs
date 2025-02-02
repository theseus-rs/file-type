use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111190469: FileFormat = FileFormat {
    id: 111_190_469,
    source_type: SourceType::Wikidata,
    name: "Adobe Extension Data Markup Language Document",
    extensions: &["edml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
