use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84942995: FileFormat = FileFormat {
    id: 84_942_995,
    source_type: SourceType::Wikidata,
    name: "Sony PictureGear Studio PrintStudio",
    extensions: &["lmd", "lmu"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x4D, 0x55, 0x44, 0x00, 0x00, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
