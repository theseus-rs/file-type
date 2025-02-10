use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852559: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_559,
        source_type: SourceType::Wikidata,
        name: "STT disk image",
        extensions: &["stt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x54, 0x45, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
