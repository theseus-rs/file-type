use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851027: FileFormat = FileFormat {
    id: 105_851_027,
    puid: "wikidata/105851027",
    name: "T81 EightyOne tape image",
    extensions: &["t81"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x4F, 0x38, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
