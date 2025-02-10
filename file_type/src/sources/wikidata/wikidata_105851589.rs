use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851589: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_589,
        source_type: SourceType::Wikidata,
        name: "Thrustmaster TARGET script",
        extensions: &["tmc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x69, 0x6E, 0x63, 0x6C, 0x75, 0x64, 0x65, 0x20, 0x22, 0x74, 0x61, 0x72,
                        0x67, 0x65, 0x74, 0x2E, 0x74, 0x6D, 0x68, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
