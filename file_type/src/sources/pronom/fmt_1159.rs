use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1159: FileFormat = FileFormat {
    id: 1_969,
    puid: "fmt/1159",
    name: "Folio Infobase File",
    extensions: &["nfo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(224),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x04, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0xFC, 0xAE, 0x56, 0x89, 0x62, 0x74, 0xBF, 0xAE,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
