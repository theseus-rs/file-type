use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762866: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_866,
        source_type: SourceType::Wikidata,
        name: "XFIT XDD format data file",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2A, 0x0D, 0x0A, 0x53, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x49, 0x64,
                        0x65, 0x6E, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
