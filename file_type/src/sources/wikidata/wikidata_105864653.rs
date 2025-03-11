use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864653: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_653,
        source_type: SourceType::Wikidata,
        name: "PageFlipper Plus FX effect",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFF, 0xFF, 0x0A, 0x41, 0x4C, 0x47, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
