use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_741654: FileType = FileType {
    file_format: &FileFormat {
        id: 741_654,
        source_type: SourceType::Wikidata,
        name: "DirectDraw Surface",
        extensions: &["dds"],
        media_types: &["image/vnd-ms.dds", "image/x-direct-draw-surface"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x44, 0x53, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
