use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858741: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_741,
        source_type: SourceType::Wikidata,
        name: "MIME Base64 encoded GIF bitmap",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x30, 0x6C, 0x47, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
