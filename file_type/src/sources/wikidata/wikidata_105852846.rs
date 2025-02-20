use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852846: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_846,
        source_type: SourceType::Wikidata,
        name: "SQR script",
        extensions: &["sqr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
