use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762865: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_865,
        source_type: SourceType::Wikidata,
        name: "IBM i Access Client Translation Table",
        extensions: &[],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x6C, 0x65, 0x5D, 0x0D, 0x0A, 0x69,
                        0x64, 0x3D, 0x58, 0x4C, 0x54, 0x0D, 0x0A, 0x44, 0x65, 0x73, 0x63, 0x72,
                        0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
