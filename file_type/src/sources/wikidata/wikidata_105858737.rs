use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858737: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_737,
        source_type: SourceType::Wikidata,
        name: "ImageMagick Machine independent File Format bitmap",
        extensions: &["mif", "miff"],
        media_types: &["image/x-miff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x3D, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x4D, 0x61, 0x67, 0x69, 0x63,
                        0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
