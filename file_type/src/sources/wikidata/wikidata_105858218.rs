use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858218: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_218,
        source_type: SourceType::Wikidata,
        name: "EmilCont compressed file",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x6D, 0x69, 0x6C, 0x63, 0x6F, 0x6E, 0x74, 0x20, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
