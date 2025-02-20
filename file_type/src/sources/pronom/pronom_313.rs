use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_313: FileType = FileType {
    file_format: &FileFormat {
        id: 313,
        source_type: SourceType::Pronom,
        name: "MapBrowser/MapWriter Vector Map Data",
        extensions: &["cbd"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x20, 0x77, 0x00, 0x02, 0x40])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x20, 0x77, 0x00, 0x33])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
