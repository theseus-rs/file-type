use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857716: FileFormat = FileFormat {
    id: 105_857_716,
    puid: "wikidata/105857716",
    name: "QuarkImmedia Document",
    extensions: &["imd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x44, 0x4F, 0x43, 0x51, 0x4F, 0x52, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
