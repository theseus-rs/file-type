use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857399: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_399,
        source_type: SourceType::Wikidata,
        name: "Lemur Module",
        extensions: &["jzlib"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x4A, 0x5A, 0x4D, 0x4C, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
