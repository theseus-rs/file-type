use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85413270: FileType = FileType {
    file_format: &FileFormat {
        id: 85_413_270,
        source_type: SourceType::Wikidata,
        name: "PTGui Project File 11",
        extensions: &["pts"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
