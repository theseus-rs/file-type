use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854676: FileFormat = FileFormat {
    id: 105_854_676,
    puid: "wikidata/105854676",
    name: "Quake archive",
    extensions: &["pak"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x43, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
