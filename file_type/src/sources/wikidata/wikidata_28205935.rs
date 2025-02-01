use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205935: FileFormat = FileFormat {
    id: 28_205_935,
    puid: "wikidata/28205935",
    name: "Doodle! compressed image",
    extensions: &["jj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x5C, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
