use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851408: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_408,
        source_type: SourceType::Wikidata,
        name: "TQSLCert request",
        extensions: &["tq5"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x74, 0x51, 0x53, 0x4C, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66,
                        0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
                        0x74, 0x0A, 0x0A, 0x3C, 0x65, 0x6F, 0x68, 0x3E, 0x0A, 0x0A, 0x3C, 0x54,
                        0x51, 0x53, 0x4C, 0x5F, 0x49, 0x44, 0x45, 0x4E, 0x54, 0x3A, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
