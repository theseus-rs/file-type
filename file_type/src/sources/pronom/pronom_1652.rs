use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1652: FileType = FileType {
    file_format: &FileFormat {
        id: 1_652,
        source_type: SourceType::Pronom,
        name: "Genealogical Data Communication (GEDCOM) Format",
        extensions: &["ged"],
        media_types: &[],
        signatures: &[Signature {
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
    },
};
