use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855749: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_749,
        source_type: SourceType::Wikidata,
        name: "Digital-FM module",
        extensions: &["dfm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x46, 0x4D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
