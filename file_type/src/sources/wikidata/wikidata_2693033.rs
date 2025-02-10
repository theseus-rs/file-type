use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_2693033: FileType = FileType {
    file_format: &FileFormat {
        id: 2_693_033,
        source_type: SourceType::Wikidata,
        name: "ARJ",
        extensions: &["arj"],
        media_types: &["application/arj"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x60, 0xEA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
