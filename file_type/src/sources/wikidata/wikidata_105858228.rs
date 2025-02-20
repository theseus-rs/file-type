use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858228: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_228,
        source_type: SourceType::Wikidata,
        name: "EverNote database",
        extensions: &["enb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4E, 0x42, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
