use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860617: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_617,
        source_type: SourceType::Wikidata,
        name: "Astra Report",
        extensions: &["rjs"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x41, 0x53, 0x54, 0x52, 0x41, 0x5D, 0x0D, 0x0A, 0x41, 0x53, 0x54,
                        0x52, 0x41, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
