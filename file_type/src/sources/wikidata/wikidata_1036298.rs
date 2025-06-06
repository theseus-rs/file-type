use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1036298: FileType = FileType {
    file_format: &FileFormat {
        id: 1_036_298,
        source_type: SourceType::Wikidata,
        name: "HighMAT",
        extensions: &["hmt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4D, 0x54, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
