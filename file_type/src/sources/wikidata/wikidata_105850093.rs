use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850093: FileFormat = FileFormat {
    id: 105_850_093,
    puid: "wikidata/105850093",
    name: "EISA add-on card Configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4F, 0x41, 0x52, 0x44, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
