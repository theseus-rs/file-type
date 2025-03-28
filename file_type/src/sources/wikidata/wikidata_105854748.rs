use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854748: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_748,
        source_type: SourceType::Wikidata,
        name: "IXimager forensic image",
        extensions: &["asb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7F, 0x52, 0x69, 0x50, 0x65, 0x64, 0x5F, 0x42, 0x79, 0x5F, 0x49, 0x4C,
                        0x6F, 0x6F, 0x6B, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
