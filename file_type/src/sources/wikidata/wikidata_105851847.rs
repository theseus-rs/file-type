use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851847: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_847,
        source_type: SourceType::Wikidata,
        name: "SBStudio II sounds",
        extensions: &["sou"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4E, 0x44, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
