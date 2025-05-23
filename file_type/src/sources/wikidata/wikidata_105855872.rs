use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855872: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_872,
        source_type: SourceType::Wikidata,
        name: "Code::Blocks Dependencies",
        extensions: &["depend"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x64, 0x65, 0x70, 0x73, 0x6C, 0x69, 0x62, 0x20, 0x64, 0x65,
                        0x70, 0x65, 0x6E, 0x64, 0x65, 0x6E, 0x63, 0x79, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x20, 0x76, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
