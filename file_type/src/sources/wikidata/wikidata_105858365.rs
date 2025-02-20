use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858365: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_365,
        source_type: SourceType::Wikidata,
        name: "E-Tracker chiptune",
        extensions: &["cop", "et", "etc", "t"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x20, 0x28, 0x43, 0x29,
                        0x20, 0x42, 0x59, 0x20, 0x45, 0x53, 0x49, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
