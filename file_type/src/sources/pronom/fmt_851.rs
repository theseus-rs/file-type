use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_851: FileFormat = FileFormat {
    id: 1_652,
    puid: "fmt/851",
    name: "Genealogical Data Communication (GEDCOM) Format",
    extensions: &["ged"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x30, 0x20, 0x48, 0x45, 0x41, 0x44]),
                    Token::WildcardCountRange(0, 1_024),
                    Token::Literal(&[0x47, 0x45, 0x44, 0x43]),
                    Token::Any(&[
                        &[Token::Literal(&[0x0D, 0x0A])],
                        &[Token::Literal(&[0x0D])],
                        &[Token::Literal(&[0x0A])],
                    ]),
                    Token::Literal(&[0x32, 0x20, 0x56, 0x45, 0x52, 0x53]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
