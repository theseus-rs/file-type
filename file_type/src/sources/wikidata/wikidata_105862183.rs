use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862183: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_183,
        source_type: SourceType::Wikidata,
        name: "Misfit Model 3D model",
        extensions: &["mm3d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x49, 0x53, 0x46, 0x49, 0x54, 0x33, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
