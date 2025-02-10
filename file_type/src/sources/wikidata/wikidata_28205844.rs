use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205844: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_844,
        source_type: SourceType::Wikidata,
        name: "COKE",
        extensions: &["tg1"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x4B, 0x45, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
