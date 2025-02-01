use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207167: FileFormat = FileFormat {
    id: 28_207_167,
    puid: "wikidata/28207167",
    name: "Piecewise-Constant Image Model",
    extensions: &["pwc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0x79, 0x56, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
