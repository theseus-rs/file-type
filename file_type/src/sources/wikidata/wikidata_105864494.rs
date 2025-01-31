use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864494: FileFormat = FileFormat {
    id: 105_864_494,
    puid: "wikidata/105864494",
    name: "PageSetter II page (v1.x)",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1D, 0x19, 0x18, 0x17, 0x50, 0x61, 0x67, 0x65, 0x53, 0x65, 0x74, 0x74, 0x65,
                    0x72, 0x20, 0x49, 0x49, 0x20, 0x28, 0x56, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
