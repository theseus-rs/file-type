use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861373: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_373,
        source_type: SourceType::Wikidata,
        name: "Linux Software Map entry (v3)",
        extensions: &["lsm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x65, 0x67, 0x69, 0x6E, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
