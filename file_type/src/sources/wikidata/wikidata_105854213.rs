use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854213: FileFormat = FileFormat {
    id: 105_854_213,
    puid: "wikidata/105854213",
    name: "BRender ASF",
    extensions: &["asf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x53, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
