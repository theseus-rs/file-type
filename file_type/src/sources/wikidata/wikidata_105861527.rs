use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861527: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_527,
        source_type: SourceType::Wikidata,
        name: "HijackThis logfile",
        extensions: &["log"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x6F, 0x67, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x6F, 0x66, 0x20, 0x54,
                        0x72, 0x65, 0x6E, 0x64, 0x20, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x20, 0x48,
                        0x69, 0x6A, 0x61, 0x63, 0x6B, 0x54, 0x68, 0x69, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
