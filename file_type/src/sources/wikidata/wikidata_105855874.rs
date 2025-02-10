use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855874: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_874,
        source_type: SourceType::Wikidata,
        name: "Cubic Player module information data base",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x75, 0x62, 0x69, 0x63, 0x20, 0x50, 0x6C, 0x61, 0x79, 0x65, 0x72,
                        0x20, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x20, 0x49, 0x6E, 0x66, 0x6F,
                        0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x44, 0x61, 0x74, 0x61,
                        0x20, 0x42, 0x61, 0x73, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
