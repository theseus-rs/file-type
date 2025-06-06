use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860903: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_903,
        source_type: SourceType::Wikidata,
        name: "MicroStation Symbology Resources",
        extensions: &["rsc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x53, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                        0x20, 0x53, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x6F, 0x67, 0x79, 0x20, 0x52,
                        0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x73, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
