use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851953: FileFormat = FileFormat {
    id: 105_851_953,
    puid: "wikidata/105851953",
    name: "PC88/PC9801 sound logs rip",
    extensions: &["s98"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x39, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
