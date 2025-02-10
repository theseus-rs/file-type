use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859896: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_896,
        source_type: SourceType::Wikidata,
        name: "Visual Sciences Resource",
        extensions: &["vsr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x52, 0x49, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
