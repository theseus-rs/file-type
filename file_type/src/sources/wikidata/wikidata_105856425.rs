use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856425: FileFormat = FileFormat {
    id: 105_856_425,
    puid: "wikidata/105856425",
    name: "GFA Raytrace Animation (low-res)",
    extensions: &["wal"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x61, 0x6C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
