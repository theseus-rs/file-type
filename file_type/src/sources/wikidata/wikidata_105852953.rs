use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852953: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_953,
        source_type: SourceType::Wikidata,
        name: "Spring Map Definition",
        extensions: &["smd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x4D, 0x41, 0x50, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
