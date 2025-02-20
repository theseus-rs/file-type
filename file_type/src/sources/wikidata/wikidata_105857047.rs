use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857047: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_047,
        source_type: SourceType::Wikidata,
        name: "GameShark SharkSave for GameCube",
        extensions: &["gcs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x43, 0x53, 0x41, 0x56, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
