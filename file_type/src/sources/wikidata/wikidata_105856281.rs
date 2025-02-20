use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856281: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_281,
        source_type: SourceType::Wikidata,
        name: "GLBasic 3D data",
        extensions: &["ddd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x17, 0xB3, 0xC6, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
