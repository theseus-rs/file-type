use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_33517407: FileType = FileType {
    file_format: &FileFormat {
        id: 33_517_407,
        source_type: SourceType::Wikidata,
        name: "E57 Lidar Point Cloud Format",
        extensions: &["e57"],
        media_types: &["model/e57"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x53, 0x54, 0x4D, 0x2D, 0x45, 0x35, 0x37,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
