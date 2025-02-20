use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859513: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_513,
        source_type: SourceType::Wikidata,
        name: "LZA animation/video",
        extensions: &["lza"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x5A, 0x41, 0x4E, 0x49, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
