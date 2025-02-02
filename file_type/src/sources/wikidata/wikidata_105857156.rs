use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857156: FileFormat = FileFormat {
    id: 105_857_156,
    source_type: SourceType::Wikidata,
    name: "Hammer compressed",
    extensions: &["hmr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x6D, 0x72, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
