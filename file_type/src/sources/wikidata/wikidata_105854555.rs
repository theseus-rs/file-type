use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854555: FileFormat = FileFormat {
    id: 105_854_555,
    puid: "wikidata/105854555",
    name: "Adventure Game eXecutable",
    extensions: &["agx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0xC7, 0xC1, 0x51, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
