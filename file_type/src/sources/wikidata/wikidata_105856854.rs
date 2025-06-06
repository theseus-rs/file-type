use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856854: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_854,
        source_type: SourceType::Wikidata,
        name: "The Games Factory Game (P)",
        extensions: &["gam"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x50, 0x50, 0x07, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
