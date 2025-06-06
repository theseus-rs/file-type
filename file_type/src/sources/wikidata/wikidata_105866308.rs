use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866308: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_308,
        source_type: SourceType::Wikidata,
        name: "BIS P3D MLOD model",
        extensions: &["p3d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4C, 0x4F, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
