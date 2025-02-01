use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853817: FileFormat = FileFormat {
    id: 105_853_817,
    puid: "wikidata/105853817",
    name: "UNIX Compressed data",
    extensions: &["z"],
    media_types: &["application/x-compress"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x9D, 0x90])],
            },
        }],
    }],
    related_formats: &[],
};
