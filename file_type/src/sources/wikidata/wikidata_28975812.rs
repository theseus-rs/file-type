use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975812: FileFormat = FileFormat {
    id: 28_975_812,
    puid: "wikidata/28975812",
    name: "International Laser Display Association image data transfer format",
    extensions: &["ild"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4C, 0x44, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
