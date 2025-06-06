use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867236: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_236,
        source_type: SourceType::Wikidata,
        name: "JB BAHN layout",
        extensions: &["nt3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
