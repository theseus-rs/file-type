use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850902: FileFormat = FileFormat {
    id: 105_850_902,
    puid: "wikidata/105850902",
    name: "KeyKit Page",
    extensions: &["kp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x67, 0x65, 0x6F, 0x62, 0x6A, 0x3D, 0x24, 0x31, 0x0A, 0x50, 0x61,
                    0x67, 0x65, 0x6E, 0x6D, 0x3D, 0x22, 0x50, 0x61, 0x67, 0x65, 0x20, 0x31, 0x22,
                    0x0A, 0x50, 0x61, 0x67, 0x65, 0x73, 0x7A, 0x3D, 0x5B, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
