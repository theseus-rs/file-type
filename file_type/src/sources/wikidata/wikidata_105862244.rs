use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862244: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_244,
        source_type: SourceType::Wikidata,
        name: "Blizzard 3D Model (binary)",
        extensions: &["mdx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x44, 0x4C, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
