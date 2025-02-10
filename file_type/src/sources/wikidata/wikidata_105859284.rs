use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859284: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_284,
        source_type: SourceType::Wikidata,
        name: "Tales Of Eternia Online game data archive (v1)",
        extensions: &["bnd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4E, 0x4B, 0x44, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
