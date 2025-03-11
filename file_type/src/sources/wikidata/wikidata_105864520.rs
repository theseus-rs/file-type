use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864520: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_520,
        source_type: SourceType::Wikidata,
        name: "Pro/ENGINEER file (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x55, 0x47, 0x43, 0x3A, 0x32, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
