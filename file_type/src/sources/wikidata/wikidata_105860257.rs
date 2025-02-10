use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860257: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_257,
        source_type: SourceType::Wikidata,
        name: "RcCad model",
        extensions: &["rcd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x52, 0x43, 0x43, 0x41, 0x44, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x56, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
