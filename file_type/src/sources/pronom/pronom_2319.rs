use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2319: FileType = FileType {
    file_format: &FileFormat {
        id: 2_319,
        source_type: SourceType::Pronom,
        name: "ZoomBrowser Ex Thumbnail Cache",
        extensions: &["info"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7A, 0x62, 0x65, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
