use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_74021299: FileType = FileType {
    file_format: &FileFormat {
        id: 74_021_299,
        source_type: SourceType::Wikidata,
        name: "Raw Voxel format",
        extensions: &["rawvox"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x56, 0x4F, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
