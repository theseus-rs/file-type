use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2755: FileType = FileType {
    file_format: &FileFormat {
        id: 2_755,
        source_type: SourceType::Pronom,
        name: "RIS Citation",
        extensions: &["ris"],
        media_types: &["application/x-research-info-systems"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x54, 0x59, 0x20])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x45, 0x52, 0x20, 0x20, 0x2D])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
