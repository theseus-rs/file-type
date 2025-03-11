use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859146: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_146,
        source_type: SourceType::Wikidata,
        name: "MIME Base64 encoded PNG bitmap",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x56, 0x42, 0x4F, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
