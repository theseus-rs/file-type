use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860358: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_358,
        source_type: SourceType::Wikidata,
        name: "Deluxe Ski Jump 2 Replay",
        extensions: &["rpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x14, 0xB8, 0xA9, 0x19, 0x89, 0xFA, 0x7A, 0xED, 0x86, 0xB8, 0x37, 0x3C,
                        0x75,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
