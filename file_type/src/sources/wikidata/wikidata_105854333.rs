use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854333: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_333,
        source_type: SourceType::Wikidata,
        name: "Authorware Packaged file (with runtime)",
        extensions: &["a4r", "a5r"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x52, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
