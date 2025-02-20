use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858717: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_717,
        source_type: SourceType::Wikidata,
        name: "BOLT game data archive",
        extensions: &["blt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4F, 0x4C, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
