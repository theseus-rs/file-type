use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850910: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_910,
        source_type: SourceType::Wikidata,
        name: "Doobs database",
        extensions: &["kdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD0, 0x0B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
