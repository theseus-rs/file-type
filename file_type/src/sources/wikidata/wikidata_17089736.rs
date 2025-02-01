use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17089736: FileFormat = FileFormat {
    id: 17_089_736,
    puid: "wikidata/17089736",
    name: "SDIF",
    extensions: &["sdif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x44, 0x49, 0x46, 0x00, 0x00, 0x00, 0x08,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
