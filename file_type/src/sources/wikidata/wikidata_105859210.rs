use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859210: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_210,
        source_type: SourceType::Wikidata,
        name: "Encrypted Blender 3D data",
        extensions: &["block"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4C, 0x4F, 0x43, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
