use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858841: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_841,
        source_type: SourceType::Wikidata,
        name: "MIME Base64 encoded PDF document",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x56, 0x42, 0x45, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
