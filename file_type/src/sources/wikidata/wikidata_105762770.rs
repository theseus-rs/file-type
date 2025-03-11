use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762770: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_770,
        source_type: SourceType::Wikidata,
        name: "Xbox 360 User Interface",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x58, 0x75, 0x69, 0x43, 0x61, 0x6E, 0x76, 0x61, 0x73, 0x20, 0x76,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
