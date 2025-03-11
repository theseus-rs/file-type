use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863840: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_840,
        source_type: SourceType::Wikidata,
        name: "MacDraft drawing (v1.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x02, 0x62, 0x02, 0x62, 0x00, 0x00,
                        0x00, 0x02, 0xB5,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
