use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858684: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_684,
        source_type: SourceType::Wikidata,
        name: "MIME Base64 encoded MP4 video",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x41, 0x41, 0x41, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
