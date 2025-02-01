use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600482: FileFormat = FileFormat {
    id: 28_600_482,
    puid: "wikidata/28600482",
    name: "DSK (Apple II)",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0xA5, 0x27, 0xC9, 0x09, 0xD0])],
            },
        }],
    }],
    related_formats: &[],
};
