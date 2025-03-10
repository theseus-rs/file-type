use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853290: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_290,
        source_type: SourceType::Wikidata,
        name: "Google Chrome saved sessions",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4E, 0x53, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
