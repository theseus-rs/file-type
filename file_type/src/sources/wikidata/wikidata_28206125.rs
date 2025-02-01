use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206125: FileFormat = FileFormat {
    id: 28_206_125,
    puid: "wikidata/28206125",
    name: "Flexible Line Interpretation",
    extensions: &["fli"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x3B])],
            },
        }],
    }],
    related_formats: &[],
};
