use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853058: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_058,
        source_type: SourceType::Wikidata,
        name: "Playmation Surface",
        extensions: &["sur"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x55, 0x52, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
