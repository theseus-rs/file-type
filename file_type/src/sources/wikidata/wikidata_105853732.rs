use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853732: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_732,
        source_type: SourceType::Wikidata,
        name: "Any Password data",
        extensions: &["apw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
