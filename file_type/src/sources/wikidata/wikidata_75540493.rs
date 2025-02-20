use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75540493: FileType = FileType {
    file_format: &FileFormat {
        id: 75_540_493,
        source_type: SourceType::Wikidata,
        name: "Ulead COOL 3D",
        extensions: &["uez"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0xD8, 0x3A, 0xF7, 0x01, 0x20, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
