use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857249: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_249,
        source_type: SourceType::Wikidata,
        name: "Hudson Soft game data (generic)",
        extensions: &["hsf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x53, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
