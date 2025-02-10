use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855842: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_842,
        source_type: SourceType::Wikidata,
        name: "deepMesh 3D Model",
        extensions: &["dpm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x00, 0x00, 0x00, 0x49, 0x44, 0x4D, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
