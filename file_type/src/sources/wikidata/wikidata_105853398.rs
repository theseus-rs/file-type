use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853398: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_398,
        source_type: SourceType::Wikidata,
        name: "Super Magicom ROM",
        extensions: &["smc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x4D, 0x45, 0x20, 0x44, 0x4F, 0x43, 0x54, 0x4F, 0x52, 0x20,
                        0x53, 0x46, 0x20, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
