use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859172: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_172,
        source_type: SourceType::Wikidata,
        name: "Binary Document container",
        extensions: &["asice", "bdoc"],
        media_types: &["application/x-bdoc"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
