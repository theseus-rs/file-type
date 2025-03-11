use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67203908: FileType = FileType {
    file_format: &FileFormat {
        id: 67_203_908,
        source_type: SourceType::Wikidata,
        name: "Molden format",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
