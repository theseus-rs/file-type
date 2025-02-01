use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859814: FileFormat = FileFormat {
    id: 105_859_814,
    puid: "wikidata/105859814",
    name: "Yog video",
    extensions: &["yog"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x4F, 0x47, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
