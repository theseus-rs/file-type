use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863847: FileFormat = FileFormat {
    id: 105_863_847,
    source_type: SourceType::Wikidata,
    name: "Microsoft Agent Character",
    extensions: &["acf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC4, 0xAB, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
