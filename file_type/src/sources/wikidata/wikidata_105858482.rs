use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858482: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_482,
        source_type: SourceType::Wikidata,
        name: "Mallard BASIC tokenized source (new)",
        extensions: &["bas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFC, 0x04, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
