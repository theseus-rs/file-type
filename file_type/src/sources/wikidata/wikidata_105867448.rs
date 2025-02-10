use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867448: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_448,
        source_type: SourceType::Wikidata,
        name: "Seifert ASCII pole figure format",
        extensions: &["nja"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x4D, 0x65, 0x61, 0x73, 0x50, 0x61, 0x72, 0x61, 0x6D, 0x65, 0x74,
                        0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
