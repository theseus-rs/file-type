use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866066: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_066,
        source_type: SourceType::Wikidata,
        name: "ProShape drawing",
        extensions: &["psp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x72, 0x6F, 0x53, 0x68, 0x61, 0x70, 0x65, 0x20, 0x44, 0x72, 0x61,
                        0x77, 0x69, 0x6E, 0x67, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
