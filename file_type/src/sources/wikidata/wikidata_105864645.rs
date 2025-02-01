use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864645: FileFormat = FileFormat {
    id: 105_864_645,
    puid: "wikidata/105864645",
    name: "PICkit 2 firmware",
    extensions: &["pk2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x32, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
