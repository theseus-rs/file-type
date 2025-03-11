use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855489: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_489,
        source_type: SourceType::Wikidata,
        name: "FATX partition image",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x54, 0x41, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
