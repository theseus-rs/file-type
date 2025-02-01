use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850384: FileFormat = FileFormat {
    id: 105_850_384,
    puid: "wikidata/105850384",
    name: "Windows Cardfile database (Unicode)",
    extensions: &["crd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4B, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
