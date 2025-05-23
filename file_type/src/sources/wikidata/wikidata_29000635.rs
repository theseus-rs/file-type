use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000635: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_635,
        source_type: SourceType::Wikidata,
        name: "PolyHedral Database",
        extensions: &["phd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3A, 0x6E, 0x61, 0x6D, 0x65, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
