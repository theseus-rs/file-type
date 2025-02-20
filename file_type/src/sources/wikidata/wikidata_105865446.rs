use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865446: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_446,
        source_type: SourceType::Wikidata,
        name: "AVZ Antiviral Toolkit data",
        extensions: &["pva"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x56, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
