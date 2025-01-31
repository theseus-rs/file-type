use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850937: FileFormat = FileFormat {
    id: 105_850_937,
    puid: "wikidata/105850937",
    name: "TTPod Skin",
    extensions: &["tsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x41, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
