use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851344: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_344,
        source_type: SourceType::Wikidata,
        name: "Abacus Time and Date data (v1.0)",
        extensions: &["tdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x69, 0x6D, 0x65, 0x44, 0x61, 0x74, 0x65, 0x20, 0x76, 0x31, 0x2E,
                        0x30, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
