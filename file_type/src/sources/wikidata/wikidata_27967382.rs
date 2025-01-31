use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967382: FileFormat = FileFormat {
    id: 27_967_382,
    puid: "wikidata/27967382",
    name: "HMI",
    extensions: &["hmi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x4D, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
