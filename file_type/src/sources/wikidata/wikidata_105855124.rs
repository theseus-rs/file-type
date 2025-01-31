use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855124: FileFormat = FileFormat {
    id: 105_855_124,
    puid: "wikidata/105855124",
    name: "PaperPort slide show",
    extensions: &["fss"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x53, 0x42, 0x4F, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
