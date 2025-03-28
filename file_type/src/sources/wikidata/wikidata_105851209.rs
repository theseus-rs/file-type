use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851209: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_209,
        source_type: SourceType::Wikidata,
        name: "TI Interactive Workbook",
        extensions: &["tii"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x49, 0x49, 0x6E, 0x74, 0x65, 0x72, 0x41, 0x63, 0x74, 0x69, 0x76,
                        0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
