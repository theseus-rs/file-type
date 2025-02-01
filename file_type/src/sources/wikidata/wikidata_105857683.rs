use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857683: FileFormat = FileFormat {
    id: 105_857_683,
    puid: "wikidata/105857683",
    name: "UDI Disk Image",
    extensions: &["udi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x44, 0x49, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
