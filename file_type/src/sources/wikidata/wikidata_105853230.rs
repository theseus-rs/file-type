use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853230: FileFormat = FileFormat {
    id: 105_853_230,
    puid: "wikidata/105853230",
    name: "Sony Foundry Video Capture project",
    extensions: &["sfvidcap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x69, 0x66, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
