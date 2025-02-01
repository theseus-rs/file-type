use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83883611: FileFormat = FileFormat {
    id: 83_883_611,
    puid: "wikidata/83883611",
    name: "Raw Flux Image",
    extensions: &["rfi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x46, 0x49, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
