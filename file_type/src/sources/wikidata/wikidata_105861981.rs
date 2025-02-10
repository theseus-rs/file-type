use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861981: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_981,
        source_type: SourceType::Wikidata,
        name: "Trilo Tracker Macro",
        extensions: &["ma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
