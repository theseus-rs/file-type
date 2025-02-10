use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851159: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_159,
        source_type: SourceType::Wikidata,
        name: "PC-88 Tape image",
        extensions: &["t88"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x2D, 0x38, 0x38, 0x30, 0x31, 0x20, 0x54, 0x61, 0x70, 0x65,
                        0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x28, 0x54, 0x38, 0x38, 0x29,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
