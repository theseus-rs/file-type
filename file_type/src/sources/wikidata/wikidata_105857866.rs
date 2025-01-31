use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857866: FileFormat = FileFormat {
    id: 105_857_866,
    puid: "wikidata/105857866",
    name: "VarioCAM thermogram",
    extensions: &["irb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x49, 0x52, 0x42, 0x00, 0x56, 0x41, 0x52, 0x49, 0x4F, 0x43, 0x41, 0x4D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
