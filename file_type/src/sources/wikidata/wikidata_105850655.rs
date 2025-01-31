use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850655: FileFormat = FileFormat {
    id: 105_850_655,
    puid: "wikidata/105850655",
    name: "KCemu tape image",
    extensions: &["kct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x43, 0x65, 0x6D, 0x75, 0x20, 0x74, 0x61, 0x70, 0x65, 0x20, 0x66, 0x69,
                    0x6C, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
