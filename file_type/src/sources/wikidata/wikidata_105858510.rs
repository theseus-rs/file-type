use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858510: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_510,
        source_type: SourceType::Wikidata,
        name: "MIME Base64 encoded JPEG bitmap",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x39, 0x6A, 0x2F, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
