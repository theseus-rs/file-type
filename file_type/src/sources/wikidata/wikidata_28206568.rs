use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206568: FileFormat = FileFormat {
    id: 28_206_568,
    puid: "wikidata/28206568",
    name: "MicroDesign Page",
    extensions: &["mdp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x4D, 0x44, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
