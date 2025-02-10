use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857070: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_070,
        source_type: SourceType::Wikidata,
        name: "Xbox Game Profile Data",
        extensions: &["gpd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x44, 0x42, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
