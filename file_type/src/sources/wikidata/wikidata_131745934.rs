use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131745934: FileType = FileType {
    file_format: &FileFormat {
        id: 131_745_934,
        source_type: SourceType::Wikidata,
        name: "Strata 3D Model",
        extensions: &["s3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x43, 0x61, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
