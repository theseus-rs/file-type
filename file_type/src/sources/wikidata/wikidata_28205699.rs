use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205699: FileFormat = FileFormat {
    id: 28_205_699,
    source_type: SourceType::Wikidata,
    name: "Aperio SVS",
    extensions: &["svs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
