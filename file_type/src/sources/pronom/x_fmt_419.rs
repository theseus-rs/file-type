use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_419: FileFormat = FileFormat {
    id: 806,
    puid: "x-fmt/419",
    name: "DVD data file and backup data file",
    extensions: &["ifo", "bup"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x56, 0x44, 0x56, 0x49, 0x44, 0x45, 0x4F, 0x2D, 0x56]),
                    Token::Any(&[
                        &[Token::Literal(&[0x4D, 0x47])],
                        &[Token::Literal(&[0x54, 0x53])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
