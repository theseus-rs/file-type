use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860699: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_699,
        source_type: SourceType::Wikidata,
        name: "Haven Resource data",
        extensions: &["res"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x61, 0x76, 0x65, 0x6E, 0x20, 0x52, 0x65, 0x73, 0x6F, 0x75, 0x72,
                        0x63, 0x65, 0x20, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
