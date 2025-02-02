use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865668: FileFormat = FileFormat {
    id: 105_865_668,
    source_type: SourceType::Wikidata,
    name: "PCSX movie capture",
    extensions: &["pxm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x58, 0x4D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
