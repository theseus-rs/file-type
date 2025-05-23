use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858417: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_417,
        source_type: SourceType::Wikidata,
        name: "PC64/DOS saved session image/dump",
        extensions: &["c64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x36, 0x34, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
