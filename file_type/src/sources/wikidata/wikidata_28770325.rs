use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770325: FileFormat = FileFormat {
    id: 28_770_325,
    puid: "wikidata/28770325",
    name: "Lepton",
    extensions: &["lep"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCF, 0x84])],
            },
        }],
    }],
    related_formats: &[],
};
