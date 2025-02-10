use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859200: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_200,
        source_type: SourceType::Wikidata,
        name: "GETIC 3D BSP",
        extensions: &["bsp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x47, 0x42, 0x54, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
