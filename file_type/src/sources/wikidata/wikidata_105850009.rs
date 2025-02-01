use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850009: FileFormat = FileFormat {
    id: 105_850_009,
    puid: "wikidata/105850009",
    name: "Cap'n Proto schema",
    extensions: &["capnp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x30, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
