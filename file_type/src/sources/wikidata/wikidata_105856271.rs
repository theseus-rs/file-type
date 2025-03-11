use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856271: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_271,
        source_type: SourceType::Wikidata,
        name: "Dynamic Env. Dynamic Graph data",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x59, 0x4E, 0x41, 0x4D, 0x49, 0x43, 0x20, 0x47, 0x52, 0x41, 0x50,
                        0x48, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
