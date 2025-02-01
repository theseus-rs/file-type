use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851739: FileFormat = FileFormat {
    id: 105_851_739,
    puid: "wikidata/105851739",
    name: "Phoenix save state (generic)",
    extensions: &["states"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x2D, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
