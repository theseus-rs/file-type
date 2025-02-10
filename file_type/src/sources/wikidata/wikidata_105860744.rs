use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860744: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_744,
        source_type: SourceType::Wikidata,
        name: "MSX2 ROM Image",
        extensions: &["rom"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
