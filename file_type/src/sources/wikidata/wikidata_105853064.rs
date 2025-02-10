use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853064: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_064,
        source_type: SourceType::Wikidata,
        name: "Atari ST Guide Hypertext document",
        extensions: &["hyp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x44, 0x4F, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
