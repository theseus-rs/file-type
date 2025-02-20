use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860424: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_424,
        source_type: SourceType::Wikidata,
        name: "Maxon Resource Creation Tool data",
        extensions: &["rct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
