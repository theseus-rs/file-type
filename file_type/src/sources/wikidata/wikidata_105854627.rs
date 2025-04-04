use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854627: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_627,
        source_type: SourceType::Wikidata,
        name: "Artemis Presents! document (v2.0)",
        extensions: &["apr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x54, 0x52, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
