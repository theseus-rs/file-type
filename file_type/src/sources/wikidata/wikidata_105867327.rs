use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867327: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_327,
        source_type: SourceType::Wikidata,
        name: "NeXT typedstream serialized data (BE)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x74, 0x79, 0x70, 0x65, 0x64, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
