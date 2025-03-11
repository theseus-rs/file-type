use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850776: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_776,
        source_type: SourceType::Wikidata,
        name: "Microsoft KWAJ compressed (Phil Katz's 'deflate')",
        extensions: &[],
        media_types: &["application/x-ms-compress-kwaj"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x57, 0x41, 0x4A, 0x88, 0xF0, 0x27, 0xD1, 0x03, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
