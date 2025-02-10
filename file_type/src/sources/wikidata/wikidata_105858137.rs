use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858137: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_137,
        source_type: SourceType::Wikidata,
        name: "Partimage disk image",
        extensions: &["img"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x52, 0x74, 0x49, 0x6D, 0x41, 0x67, 0x45, 0x2D, 0x56, 0x6F,
                        0x4C, 0x75, 0x4D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
