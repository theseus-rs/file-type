use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_47538013: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_013,
        source_type: SourceType::Wikidata,
        name: "CDX Internet Archive Index format",
        extensions: &["cdx"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x58, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
