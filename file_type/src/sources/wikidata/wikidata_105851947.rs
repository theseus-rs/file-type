use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851947: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_947,
        source_type: SourceType::Wikidata,
        name: "Mortal Kombat serie game data archive",
        extensions: &["ssf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x43, 0x45, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
