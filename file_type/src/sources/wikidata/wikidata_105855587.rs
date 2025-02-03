use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855587: FileFormat = FileFormat {
    id: 105_855_587,
    source_type: SourceType::Wikidata,
    name: "Turbo Pascal Overlay",
    extensions: &["ovr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x4F, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
