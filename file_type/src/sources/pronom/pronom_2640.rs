use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2640: FileFormat = FileFormat {
    id: 2_640,
    source_type: SourceType::Pronom,
    name: "Help Librarian File",
    extensions: &["hlp", "dat", "dta"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x2D, 0x57, 0x6F, 0x72, 0x74, 0x68, 0x79, 0x20, 0x28, 0x52, 0x29, 0x20,
                    0x48, 0x65, 0x6C, 0x70, 0x20, 0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x69, 0x61,
                    0x6E, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56,
                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
