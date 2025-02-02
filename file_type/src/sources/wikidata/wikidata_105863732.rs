use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863732: FileFormat = FileFormat {
    id: 105_863_732,
    source_type: SourceType::Wikidata,
    name: "m4 preprocessor / macro source",
    extensions: &["m4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x6E, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
