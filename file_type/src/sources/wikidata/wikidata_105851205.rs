use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851205: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_205,
        source_type: SourceType::Wikidata,
        name: "Xoom Tutor tutorial",
        extensions: &["tut"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x20, 0x69, 0x6E, 0x6D, 0x65, 0x6E,
                        0x75, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
