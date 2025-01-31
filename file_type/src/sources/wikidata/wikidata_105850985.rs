use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850985: FileFormat = FileFormat {
    id: 105_850_985,
    puid: "wikidata/105850985",
    name: "Teledisk Disk compressed image (advanced mode)",
    extensions: &["td0"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x64, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
