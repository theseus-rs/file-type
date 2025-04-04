use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861158: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_158,
        source_type: SourceType::Wikidata,
        name: "IBM Works for OS/2 word processor document",
        extensions: &["lwp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x42, 0x4D, 0x20, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x20, 0x4F, 0x53,
                        0x2F, 0x32, 0x00, 0x00, 0x57, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
