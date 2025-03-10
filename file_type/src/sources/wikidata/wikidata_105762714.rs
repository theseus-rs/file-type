use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762714: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_714,
        source_type: SourceType::Wikidata,
        name: "XM7 snapshot",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x4D, 0x37, 0x20, 0x56, 0x4D, 0x20, 0x53, 0x54, 0x41, 0x54, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
