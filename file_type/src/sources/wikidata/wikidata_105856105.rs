use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856105: FileFormat = FileFormat {
    id: 105_856_105,
    puid: "wikidata/105856105",
    name: "DNG Camera Profile",
    extensions: &["dcp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x43, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
