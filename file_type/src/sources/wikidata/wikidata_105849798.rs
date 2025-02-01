use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849798: FileFormat = FileFormat {
    id: 105_849_798,
    puid: "wikidata/105849798",
    name: "Capella gallery data file",
    extensions: &["cag"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x61, 0x70, 0x33, 0x2D, 0x63, 0x61, 0x67,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
