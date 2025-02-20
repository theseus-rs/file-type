use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866041: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_041,
        source_type: SourceType::Wikidata,
        name: "Lotus Picture",
        extensions: &["pic"],
        media_types: &["image/x-lotus-pic"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x44, 0x00, 0x00, 0x00,
                        0x00, 0x0C, 0x7F, 0x09, 0x06,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
