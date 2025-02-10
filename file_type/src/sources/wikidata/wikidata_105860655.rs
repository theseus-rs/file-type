use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860655: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_655,
        source_type: SourceType::Wikidata,
        name: "Robinson Technologies Textures",
        extensions: &["rttex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x54, 0x50, 0x41, 0x43, 0x4B, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
