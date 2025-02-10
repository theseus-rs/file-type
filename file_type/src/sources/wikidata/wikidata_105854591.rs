use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854591: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_591,
        source_type: SourceType::Wikidata,
        name: "Limit compressed archive",
        extensions: &["lim"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
