use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867617: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_617,
        source_type: SourceType::Wikidata,
        name: "Expert Help hypertext",
        extensions: &["ng"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x48, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
