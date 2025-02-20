use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858646: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_646,
        source_type: SourceType::Wikidata,
        name: "Boundary Scan Description Language",
        extensions: &["bsdl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x65, 0x6E, 0x74, 0x69, 0x74, 0x79, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
