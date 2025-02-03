use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855330: FileFormat = FileFormat {
    id: 105_855_330,
    source_type: SourceType::Wikidata,
    name: "Full Disk Image",
    extensions: &["fdi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x44, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
