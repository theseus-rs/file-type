use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853598: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_598,
        source_type: SourceType::Wikidata,
        name: "Minecraft game data",
        extensions: &["zipe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0xFC, 0xB9, 0xCF, 0x9B, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x24,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
