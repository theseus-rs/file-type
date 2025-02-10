use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855646: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_646,
        source_type: SourceType::Wikidata,
        name: "Radiance Octree",
        extensions: &["oct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x3F, 0x52, 0x41, 0x44, 0x49, 0x41, 0x4E, 0x43, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
