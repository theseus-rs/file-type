use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1801: FileFormat = FileFormat {
    id: 2_652,
    puid: "fmt/1801",
    name: "Praat TextGrid",
    extensions: &["textgrid"],
    media_types: &["text/praat-textgrid"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x3D, 0x20,
                        0x22, 0x6F, 0x6F, 0x54, 0x65, 0x78, 0x74, 0x46, 0x69, 0x6C, 0x65, 0x22,
                    ]),
                    Token::WildcardCountRange(1, 3),
                    Token::Literal(&[
                        0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x63, 0x6C, 0x61, 0x73, 0x73,
                        0x20, 0x3D, 0x20, 0x22, 0x54, 0x65, 0x78, 0x74, 0x47, 0x72, 0x69, 0x64,
                        0x22,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
