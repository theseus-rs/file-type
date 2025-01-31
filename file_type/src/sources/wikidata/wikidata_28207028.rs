use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207028: FileFormat = FileFormat {
    id: 28_207_028,
    puid: "wikidata/28207028",
    name: "Pixia",
    extensions: &["pxa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x69, 0x78, 0x69, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
