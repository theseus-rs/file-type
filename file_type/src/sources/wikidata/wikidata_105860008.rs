use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860008: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_008,
        source_type: SourceType::Wikidata,
        name: "Vortex Tracker 2 chiptune",
        extensions: &["vt2"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x6F, 0x72, 0x74, 0x65, 0x78, 0x20, 0x54, 0x72, 0x61, 0x63, 0x6B,
                        0x65, 0x72, 0x20, 0x49, 0x49, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
