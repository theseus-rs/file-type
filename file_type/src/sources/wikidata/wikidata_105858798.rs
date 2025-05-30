use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858798: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_798,
        source_type: SourceType::Wikidata,
        name: "Wii Effects Textures",
        extensions: &["breft"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x45, 0x46, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
