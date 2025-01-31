use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851442: FileFormat = FileFormat {
    id: 105_851_442,
    puid: "wikidata/105851442",
    name: "TrueType/OpenType Font Collection (generic)",
    extensions: &["otc", "ttc"],
    media_types: &["font/collection", "font/collection"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x74, 0x63, 0x66])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x74, 0x63, 0x66])],
                },
            }],
        },
    ],
    related_formats: &[],
};
