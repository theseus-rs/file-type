use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866833: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_833,
        source_type: SourceType::Wikidata,
        name: "deepMesh 3D PreFab",
        extensions: &["pf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x00, 0x00, 0x00, 0x49, 0x44, 0x4D, 0x5F, 0x50, 0x46, 0x5F, 0x31,
                        0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
