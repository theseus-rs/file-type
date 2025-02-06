use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111417217: FileFormat = FileFormat {
    id: 111_417_217,
    source_type: SourceType::Wikidata,
    name: "Assembly Language Source Code File",
    extensions: &["asm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
