use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856041: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_041,
        source_type: SourceType::Wikidata,
        name: "Steam Downloadable Content info",
        extensions: &["dlc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6E, 0x61, 0x6D, 0x65, 0x20, 0x3D, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
