use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850298: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_298,
        source_type: SourceType::Wikidata,
        name: "Cellsprings CA Rule",
        extensions: &["car"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x52, 0x75, 0x6C, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
