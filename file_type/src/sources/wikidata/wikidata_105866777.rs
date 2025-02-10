use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866777: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_777,
        source_type: SourceType::Wikidata,
        name: "Glyphs outline hashes",
        extensions: &["processedhashmap"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x0A, 0x27])],
                },
            }],
        }],
        related_formats: &[],
    },
};
