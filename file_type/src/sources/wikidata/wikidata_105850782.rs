use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850782: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_782,
        source_type: SourceType::Wikidata,
        name: "Keyman keyboard source (with rem)",
        extensions: &["kmn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x63, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
