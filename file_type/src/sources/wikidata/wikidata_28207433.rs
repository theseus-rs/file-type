use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207433: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_433,
        source_type: SourceType::Wikidata,
        name: "V.Flash PTX",
        extensions: &["ptx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2C, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
