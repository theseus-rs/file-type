use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854034: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_034,
        source_type: SourceType::Wikidata,
        name: "PAQ8JC compressed archive",
        extensions: &["paq8jc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x61, 0x71, 0x38, 0x6A, 0x63, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
