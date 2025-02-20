use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854863: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_863,
        source_type: SourceType::Wikidata,
        name: "Propellerhead Software Reason SoundBank",
        extensions: &["rfl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x72, 0x6F, 0x70, 0x65, 0x6C, 0x6C, 0x65, 0x72, 0x68, 0x65, 0x61,
                        0x64, 0x73, 0x20, 0x52, 0x65, 0x46, 0x69, 0x6C, 0x6C, 0x20, 0x46, 0x69,
                        0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
