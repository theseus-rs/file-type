use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865888: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_888,
        source_type: SourceType::Wikidata,
        name: "Gerber Photoplot",
        extensions: &["pho"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
