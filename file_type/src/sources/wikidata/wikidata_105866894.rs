use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866894: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_894,
        source_type: SourceType::Wikidata,
        name: "PWK Virtual machine module",
        extensions: &["pvm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x57, 0x4B, 0x20, 0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x20,
                        0x6D, 0x61, 0x63, 0x68, 0x69, 0x6E, 0x65, 0x20, 0x6D, 0x6F, 0x64, 0x75,
                        0x6C, 0x65, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
