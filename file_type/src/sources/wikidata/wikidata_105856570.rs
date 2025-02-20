use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856570: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_570,
        source_type: SourceType::Wikidata,
        name: "Shockwave 3D Scene Export",
        extensions: &["w3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x46, 0x58, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
