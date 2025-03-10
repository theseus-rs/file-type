use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762752: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_752,
        source_type: SourceType::Wikidata,
        name: "The Hanna-Barbera Animation Workshop animation data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x6F, 0x6D, 0x65, 0x5F, 0x4D, 0x6F, 0x76, 0x69, 0x65, 0x20, 0x20,
                        0x41, 0x4D, 0x49, 0x47, 0x41, 0x5F, 0x50, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
