use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850190: FileFormat = FileFormat {
    id: 105_850_190,
    puid: "wikidata/105850190",
    name: "Playmation Chor data",
    extensions: &["cho"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x4F, 0x52, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
