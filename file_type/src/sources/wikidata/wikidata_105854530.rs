use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854530: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_530,
        source_type: SourceType::Wikidata,
        name: "LTSpice Circuit Schematic",
        extensions: &["asc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x34, 0x0D, 0x0A, 0x53,
                        0x48, 0x45, 0x45, 0x54, 0x20, 0x31, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
