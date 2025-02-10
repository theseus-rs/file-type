use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854449: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_449,
        source_type: SourceType::Wikidata,
        name: "Distribution Package archive",
        extensions: &["pac"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x50, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
