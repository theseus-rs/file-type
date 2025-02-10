use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858119: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_119,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine WeiDU Dialogue",
        extensions: &["d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x7E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
