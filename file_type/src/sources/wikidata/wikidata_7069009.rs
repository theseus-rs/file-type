use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7069009: FileType = FileType {
    file_format: &FileFormat {
        id: 7_069_009,
        source_type: SourceType::Wikidata,
        name: "Nullsoft Streaming Video",
        extensions: &["nsv"],
        media_types: &["application/x-winamp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x53, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
