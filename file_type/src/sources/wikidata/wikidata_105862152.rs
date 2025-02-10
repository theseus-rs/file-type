use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862152: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_152,
        source_type: SourceType::Wikidata,
        name: "MacDraft drawing",
        extensions: &["mdd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x44, 0x44, 0x43, 0x32, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
