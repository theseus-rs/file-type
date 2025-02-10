use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860142: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_142,
        source_type: SourceType::Wikidata,
        name: "Rocket eBook (variant)",
        extensions: &["rb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB0, 0x0C, 0xF0, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
