use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770288: FileFormat = FileFormat {
    id: 28_770_288,
    puid: "wikidata/28770288",
    name: "LBR",
    extensions: &["lbr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x57, 0x42, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
