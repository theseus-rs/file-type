use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867502: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_502,
        source_type: SourceType::Wikidata,
        name: "Newton Toolkit Project",
        extensions: &["ntk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x02, 0x06, 0x07, 0x07, 0x0F, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74,
                        0x53, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x07, 0x0E, 0x6F, 0x75,
                        0x74, 0x70, 0x75, 0x74, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73,
                        0x07, 0x0F, 0x70, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x53, 0x65, 0x74,
                        0x74, 0x69, 0x6E, 0x67, 0x73, 0x07, 0x10, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
