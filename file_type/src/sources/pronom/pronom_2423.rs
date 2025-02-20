use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2423: FileType = FileType {
    file_format: &FileFormat {
        id: 2_423,
        source_type: SourceType::Pronom,
        name: "ESRI ArcInfo Grid .nit File",
        extensions: &["nit"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(16),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::Any(&[
                            &[Token::Literal(&[0x02])],
                            &[Token::Literal(&[0x04])],
                            &[Token::Literal(&[0x08])],
                        ]),
                        Token::Literal(&[0xFF, 0xFF, 0x00, 0x01, 0x00, 0x04]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
