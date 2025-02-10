use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851424: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_424,
        source_type: SourceType::Wikidata,
        name: "TagInfo data",
        extensions: &["tag"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x54, 0x61, 0x67, 0x49, 0x6E, 0x66, 0x6F, 0x5D, 0x0D, 0x0A, 0x43,
                        0x6F, 0x6D, 0x70, 0x61, 0x6E, 0x79, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
