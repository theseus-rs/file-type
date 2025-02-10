use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863263: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_263,
        source_type: SourceType::Wikidata,
        name: "Monarch Model",
        extensions: &["mod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x4E, 0x4D, 0x4F, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
