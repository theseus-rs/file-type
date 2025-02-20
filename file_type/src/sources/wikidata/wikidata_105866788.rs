use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866788: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_788,
        source_type: SourceType::Wikidata,
        name: "PGN (Portable Gaming Notation) Compressed format",
        extensions: &["pgc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x05])],
                },
            }],
        }],
        related_formats: &[],
    },
};
