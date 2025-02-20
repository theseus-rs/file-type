use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855994: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_994,
        source_type: SourceType::Wikidata,
        name: "Apache mission Data",
        extensions: &["dta"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3A, 0x54, 0x61, 0x73, 0x6B, 0x0D, 0x0A, 0x3A, 0x55, 0x73, 0x65, 0x72,
                        0x20, 0x54, 0x61, 0x73, 0x6B, 0x0D, 0x0A, 0x3A, 0x53, 0x69, 0x64, 0x65,
                        0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
