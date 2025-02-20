use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856898: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_898,
        source_type: SourceType::Wikidata,
        name: "Geometer's Sketchpad Document",
        extensions: &["gsp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x53, 0x50, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
