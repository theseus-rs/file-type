use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850299: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_299,
        source_type: SourceType::Wikidata,
        name: "IntroCAD drawing",
        extensions: &["cad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x12, 0xD6, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
