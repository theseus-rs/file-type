use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856802: FileFormat = FileFormat {
    id: 105_856_802,
    puid: "wikidata/105856802",
    name: "GeneRally track",
    extensions: &["trk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x47, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
