use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855763: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_763,
        source_type: SourceType::Wikidata,
        name: "Battlefield 2 mod Description",
        extensions: &["desc"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x6D, 0x6F, 0x64, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
