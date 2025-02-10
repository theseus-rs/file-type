use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859904: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_904,
        source_type: SourceType::Wikidata,
        name: "VVVV Patchlet",
        extensions: &["v4p"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x50, 0x41,
                        0x54, 0x43, 0x48, 0x20, 0x20, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D, 0x20,
                        0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x76, 0x76, 0x76, 0x76,
                        0x2E, 0x6F, 0x72, 0x67, 0x2F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x73, 0x2F, 0x76, 0x76, 0x76, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
