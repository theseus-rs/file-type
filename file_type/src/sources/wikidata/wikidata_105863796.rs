use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863796: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_796,
        source_type: SourceType::Wikidata,
        name: "7DTD player Map",
        extensions: &["map"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x61, 0x70, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
