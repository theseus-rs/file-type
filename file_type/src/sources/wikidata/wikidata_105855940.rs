use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855940: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_940,
        source_type: SourceType::Wikidata,
        name: "DCF images container",
        extensions: &["dcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4B, 0x49, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
