use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865211: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_211,
        source_type: SourceType::Wikidata,
        name: "Aegis ProMotion Set",
        extensions: &["pset"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x44, 0x50, 0x53, 0x31, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
