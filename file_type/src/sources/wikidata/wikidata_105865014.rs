use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865014: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_014,
        source_type: SourceType::Wikidata,
        name: "TomTom PNA map info",
        extensions: &["pna"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
