use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111190501: FileFormat = FileFormat {
    id: 111_190_501,
    source_type: SourceType::Wikidata,
    name: "Visual Tool Markup Language Document",
    extensions: &["vtm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
