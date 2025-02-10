use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_364,
        source_type: SourceType::Wikidata,
        name: "Datacolor QTX format (batch)",
        extensions: &["qtx"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x42, 0x41, 0x54, 0x43, 0x48, 0x5F, 0x44, 0x41, 0x54, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
