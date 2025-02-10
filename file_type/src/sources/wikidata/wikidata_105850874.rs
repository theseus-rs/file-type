use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850874: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_874,
        source_type: SourceType::Wikidata,
        name: "Keyman keyboard source",
        extensions: &["kmn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x28, 0x26, 0x56, 0x45,
                        0x52, 0x53, 0x49, 0x4F, 0x4E, 0x29, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
