use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967198: FileFormat = FileFormat {
    id: 27_967_198,
    puid: "wikidata/27967198",
    name: "Liquid Digitized Sample",
    extensions: &["lds"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x44, 0x53, 0x53, 0x01, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
