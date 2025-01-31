use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857916: FileFormat = FileFormat {
    id: 105_857_916,
    puid: "wikidata/105857916",
    name: "VMware 4 Virtual Disk (monolitic)",
    extensions: &["vmdk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x44, 0x4D, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
