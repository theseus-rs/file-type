use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863947: FileFormat = FileFormat {
    id: 105_863_947,
    source_type: SourceType::Wikidata,
    name: "Mizar article (with rem)",
    extensions: &["miz"],
    media_types: &["text/mizar"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3A, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
