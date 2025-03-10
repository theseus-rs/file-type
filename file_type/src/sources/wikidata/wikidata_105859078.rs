use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859078: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_078,
        source_type: SourceType::Wikidata,
        name: "BALTRAD data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x41, 0x4C, 0x54, 0x52, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
