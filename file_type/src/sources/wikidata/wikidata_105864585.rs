use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864585: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_585,
        source_type: SourceType::Wikidata,
        name: "PowerBuilder Dynamic library",
        extensions: &["pbd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x44, 0x52, 0x2A, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
