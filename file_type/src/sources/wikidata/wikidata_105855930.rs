use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855930: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_930,
        source_type: SourceType::Wikidata,
        name: "Dark compressed data",
        extensions: &["dark"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x64, 0x61, 0x72, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
