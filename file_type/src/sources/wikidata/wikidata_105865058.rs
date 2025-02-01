use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865058: FileFormat = FileFormat {
    id: 105_865_058,
    puid: "wikidata/105865058",
    name: "PageSetter II document (v1.x)",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x17, 0x18, 0x19, 0x1D, 0x50, 0x61, 0x67, 0x65, 0x53, 0x65, 0x74, 0x74, 0x65,
                    0x72, 0x20, 0x49, 0x49, 0x20, 0x28, 0x56, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
