use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861068: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_068,
        source_type: SourceType::Wikidata,
        name: "Look and Feel screen",
        extensions: &["lnf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4E, 0x46, 0x56, 0x45, 0x52, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
