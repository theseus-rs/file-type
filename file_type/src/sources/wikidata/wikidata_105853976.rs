use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853976: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_976,
        source_type: SourceType::Wikidata,
        name: "Amiga Money budgets (v1)",
        extensions: &["amm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x4D, 0x31, 0x42, 0x55, 0x44, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
