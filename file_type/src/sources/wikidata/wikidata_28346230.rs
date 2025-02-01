use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28346230: FileFormat = FileFormat {
    id: 28_346_230,
    puid: "wikidata/28346230",
    name: "IMD",
    extensions: &["imd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x44, 0x55, 0x0D, 0x0A, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
