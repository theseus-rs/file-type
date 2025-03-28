use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856612: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_612,
        source_type: SourceType::Wikidata,
        name: "Settlers II map",
        extensions: &["swd", "wld"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x4F, 0x52, 0x4C, 0x44, 0x5F, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
