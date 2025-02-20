use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855103: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_103,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project 4.0 for DOS Activity",
        extensions: &["act"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x41, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
