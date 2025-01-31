use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205755: FileFormat = FileFormat {
    id: 28_205_755,
    puid: "wikidata/28205755",
    name: "Big Flexible Line Interpretation",
    extensions: &["bfli"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x3B, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
