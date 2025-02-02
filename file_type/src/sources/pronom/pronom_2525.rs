use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2525: FileFormat = FileFormat {
    id: 2_525,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for MS-DOS Glossary File",
    extensions: &["gly"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x31, 0xBE, 0x01, 0x00, 0x00, 0xAB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
