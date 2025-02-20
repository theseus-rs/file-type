use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860639: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_639,
        source_type: SourceType::Wikidata,
        name: "Regressi Win data",
        extensions: &["rw3"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x56, 0x41, 0x52, 0x49, 0x53, 0x54, 0x45, 0x20, 0x52, 0x45, 0x47,
                        0x52, 0x45, 0x53, 0x53, 0x49, 0x20, 0x57, 0x49, 0x4E, 0x44, 0x4F, 0x57,
                        0x53, 0x20, 0x31, 0x2E, 0x30, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
