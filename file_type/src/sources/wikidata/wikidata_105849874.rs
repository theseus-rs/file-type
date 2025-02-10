use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849874: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_874,
        source_type: SourceType::Wikidata,
        name: "CBI document",
        extensions: &["cbi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x49, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
