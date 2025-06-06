use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762840: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_840,
        source_type: SourceType::Wikidata,
        name: "Cross-Reference info",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x76, 0x65, 0x6E, 0x64, 0x6F, 0x72, 0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x20,
                        0x3D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
