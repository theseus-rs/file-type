use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858780: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_780,
        source_type: SourceType::Wikidata,
        name: "BushTracker 2 Song",
        extensions: &["b2s"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x75, 0x73, 0x68, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x20,
                        0x32, 0x2E, 0x30, 0x20, 0x53, 0x6F, 0x6E, 0x67, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
