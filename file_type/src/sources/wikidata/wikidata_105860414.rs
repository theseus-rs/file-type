use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860414: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_414,
        source_type: SourceType::Wikidata,
        name: "ArtCAM 3D Relief model",
        extensions: &["rlf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A, 0x20, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
