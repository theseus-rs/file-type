use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867487: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_487,
        source_type: SourceType::Wikidata,
        name: "64NET container",
        extensions: &["n64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x36, 0x34, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
