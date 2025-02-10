use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205839: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_839,
        source_type: SourceType::Wikidata,
        name: "CMU Window Manager bitmap",
        extensions: &["cmu"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF1, 0x00, 0x40, 0xBB, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
