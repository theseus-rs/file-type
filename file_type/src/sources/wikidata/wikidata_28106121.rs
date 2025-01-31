use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28106121: FileFormat = FileFormat {
    id: 28_106_121,
    puid: "wikidata/28106121",
    name: "PXM",
    extensions: &["pxm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x58, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
