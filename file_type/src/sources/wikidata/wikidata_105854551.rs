use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854551: FileFormat = FileFormat {
    id: 105_854_551,
    puid: "wikidata/105854551",
    name: "AutoREALM Map",
    extensions: &["aur"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x75, 0x74, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
