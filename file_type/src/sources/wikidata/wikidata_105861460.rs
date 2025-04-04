use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861460: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_460,
        source_type: SourceType::Wikidata,
        name: "Applause Layout data",
        extensions: &["lay"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x79, 0x6F, 0x75, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                        0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
