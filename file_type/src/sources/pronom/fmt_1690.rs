use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1690: FileFormat = FileFormat {
    id: 2_526,
    puid: "fmt/1690",
    name: "Microsoft Word for MS-DOS Style Sheet File",
    extensions: &["sty"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x31, 0xBE, 0x02, 0x00, 0x00, 0xAB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
