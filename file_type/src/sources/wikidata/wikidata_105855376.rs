use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855376: FileFormat = FileFormat {
    id: 105_855_376,
    puid: "wikidata/105855376",
    name: "F.R.A.C. project",
    extensions: &["file"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x52, 0x41, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
