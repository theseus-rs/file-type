use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852771: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_771,
        source_type: SourceType::Wikidata,
        name: "saltpack encrypted message (ASCII)",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x45, 0x47, 0x49, 0x4E, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
