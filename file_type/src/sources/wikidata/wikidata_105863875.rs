use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863875: FileFormat = FileFormat {
    id: 105_863_875,
    source_type: SourceType::Wikidata,
    name: "VirtualBus Map",
    extensions: &["ms1"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
