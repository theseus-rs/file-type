use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28757997: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_997,
        source_type: SourceType::Wikidata,
        name: "In the Groove PCK",
        extensions: &["pck"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x4B, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
