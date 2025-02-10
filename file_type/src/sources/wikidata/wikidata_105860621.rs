use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860621: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_621,
        source_type: SourceType::Wikidata,
        name: "RAGE Package Format (GTA V)",
        extensions: &["rpf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x46, 0x50, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
