use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849299: FileFormat = FileFormat {
    id: 105_849_299,
    puid: "wikidata/105849299",
    name: "Yukes 3D Object",
    extensions: &["yobj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x4F, 0x42, 0x4A])],
            },
        }],
    }],
    related_formats: &[],
};
