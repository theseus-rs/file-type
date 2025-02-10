use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856232: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_232,
        source_type: SourceType::Wikidata,
        name: "Battlefield 2 map Description",
        extensions: &["desc"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x6D, 0x61, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
