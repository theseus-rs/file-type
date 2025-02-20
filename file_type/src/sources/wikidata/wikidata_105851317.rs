use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851317: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_317,
        source_type: SourceType::Wikidata,
        name: "Dynamix Track data container",
        extensions: &["trk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x52, 0x4B, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
