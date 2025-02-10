use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850144: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_144,
        source_type: SourceType::Wikidata,
        name: "TeslaCrypt/Cryptowall encrypted",
        extensions: &["ccc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDE, 0xAD, 0xBE, 0xEF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
