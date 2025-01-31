use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854573: FileFormat = FileFormat {
    id: 105_854_573,
    puid: "wikidata/105854573",
    name: "Maxis XA Audio (sound/speech)",
    extensions: &["xa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x41, 0x49, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
