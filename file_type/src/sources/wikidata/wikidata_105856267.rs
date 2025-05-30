use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856267: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_267,
        source_type: SourceType::Wikidata,
        name: "Dargon Package File",
        extensions: &["dpf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x44, 0x50, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
