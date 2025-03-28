use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858877: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_877,
        source_type: SourceType::Wikidata,
        name: "BIMx 3D scene",
        extensions: &["bimx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x49, 0x4D, 0x78, 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
                        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
