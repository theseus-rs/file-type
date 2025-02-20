use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858775: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_775,
        source_type: SourceType::Wikidata,
        name: "Tales Of Eternia Online game data archive (v2)",
        extensions: &["bnd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4E, 0x4B, 0x44, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
