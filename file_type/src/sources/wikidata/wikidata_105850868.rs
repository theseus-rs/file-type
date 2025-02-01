use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850868: FileFormat = FileFormat {
    id: 105_850_868,
    puid: "wikidata/105850868",
    name: "KSSX music format dump",
    extensions: &["kss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x53, 0x53, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
