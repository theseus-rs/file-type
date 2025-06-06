use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855546: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_546,
        source_type: SourceType::Wikidata,
        name: "OFF geometry definition",
        extensions: &["off"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
