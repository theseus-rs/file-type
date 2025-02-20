use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855056: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_056,
        source_type: SourceType::Wikidata,
        name: "Audio Sculpture module",
        extensions: &["adsc", "as"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x64, 0x69, 0x6F, 0x53, 0x63, 0x75, 0x6C, 0x70, 0x74, 0x75,
                        0x72, 0x65, 0x31, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
