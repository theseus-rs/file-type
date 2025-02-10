use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860162: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_162,
        source_type: SourceType::Wikidata,
        name: "Miasmata game data",
        extensions: &["rs5"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x46, 0x49, 0x4C, 0x45, 0x48, 0x44, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
