use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859428: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_428,
        source_type: SourceType::Wikidata,
        name: "Datacolor QTX format (standard)",
        extensions: &["qtx"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x53, 0x54, 0x41, 0x4E, 0x44, 0x41, 0x52, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
