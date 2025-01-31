use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854602: FileFormat = FileFormat {
    id: 105_854_602,
    puid: "wikidata/105854602",
    name: "FMOD Sample Bank format (generic)",
    extensions: &["fsb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
