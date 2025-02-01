use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860099: FileFormat = FileFormat {
    id: 105_860_099,
    puid: "wikidata/105860099",
    name: "FontLab Font",
    extensions: &["vfb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0x57, 0x4C, 0x46, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
