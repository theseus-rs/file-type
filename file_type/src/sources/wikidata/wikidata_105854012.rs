use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854012: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_012,
        source_type: SourceType::Wikidata,
        name: "Freeze! compressed archive",
        extensions: &["ice"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
