use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850987: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_987,
        source_type: SourceType::Wikidata,
        name: "Typed Voxel format",
        extensions: &["tox"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x6F, 0x78, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
