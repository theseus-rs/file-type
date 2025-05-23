use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5165072: FileType = FileType {
    file_format: &FileFormat {
        id: 5_165_072,
        source_type: SourceType::Wikidata,
        name: "Content Sealed Format",
        extensions: &["csf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6C, 0x00, 0x00, 0x00, 0x49, 0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                        0x69, 0x76, 0x65, 0x20, 0x47, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x73,
                        0x20, 0x43, 0x6F, 0x72, 0x70, 0x2E, 0x20, 0x53, 0x65, 0x63, 0x75, 0x72,
                        0x65, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x20, 0x46, 0x6F,
                        0x72, 0x6D, 0x61, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
