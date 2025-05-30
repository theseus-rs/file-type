use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863148: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_148,
        source_type: SourceType::Wikidata,
        name: "Coconizer new module format (v1)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x6F, 0x63, 0x6F, 0x01, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
