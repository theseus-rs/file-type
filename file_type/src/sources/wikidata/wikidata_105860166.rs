use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860166: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_166,
        source_type: SourceType::Wikidata,
        name: "REALbasic Form/Window",
        extensions: &["rbfrm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x74, 0x61, 0x67, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
