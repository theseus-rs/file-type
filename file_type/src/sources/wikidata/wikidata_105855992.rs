use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855992: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_992,
        source_type: SourceType::Wikidata,
        name: "DA's Layout project",
        extensions: &["dip"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x41, 0x4C, 0x41, 0x59, 0x4F, 0x55, 0x54, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
