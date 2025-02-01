use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850810: FileFormat = FileFormat {
    id: 105_850_810,
    puid: "wikidata/105850810",
    name: "Voxlap Frame Animation",
    extensions: &["kfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x77, 0x6C, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
