use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864090: FileFormat = FileFormat {
    id: 105_864_090,
    puid: "wikidata/105864090",
    name: "Yamaha BULK Manager Messages",
    extensions: &["msg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x4D, 0x53, 0x42, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
