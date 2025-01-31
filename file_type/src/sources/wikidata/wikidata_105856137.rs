use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856137: FileFormat = FileFormat {
    id: 105_856_137,
    puid: "wikidata/105856137",
    name: "TS Online modems definitions",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x4D, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
