use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858703: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_703,
        source_type: SourceType::Wikidata,
        name: "Magic Painter bitmap",
        extensions: &["mgp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF4, 0x0E, 0x36, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
