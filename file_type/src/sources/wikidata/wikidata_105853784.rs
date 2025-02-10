use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_784,
        source_type: SourceType::Wikidata,
        name: "SKY compressed archive",
        extensions: &["sky"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xBC, 0x40, 0x10, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
