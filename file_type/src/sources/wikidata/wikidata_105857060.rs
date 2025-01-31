use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857060: FileFormat = FileFormat {
    id: 105_857_060,
    puid: "wikidata/105857060",
    name: "Moebius Graphics Library",
    extensions: &["glb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4C, 0x42, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
