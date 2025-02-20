use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856924: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_924,
        source_type: SourceType::Wikidata,
        name: "Geometer's Sketchpad Script",
        extensions: &["gss"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x53, 0x50, 0x63, 0x0E, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
