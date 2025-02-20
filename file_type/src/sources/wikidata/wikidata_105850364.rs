use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_364,
        source_type: SourceType::Wikidata,
        name: "Canon Design Essentials printer info",
        extensions: &["csc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x63, 0x73, 0x63, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
