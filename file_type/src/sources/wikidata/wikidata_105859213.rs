use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859213: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_213,
        source_type: SourceType::Wikidata,
        name: "MIME Base64 encoded BMP bitmap",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x6B, 0x30, 0x32, 0x55])],
                },
            }],
        }],
        related_formats: &[],
    },
};
