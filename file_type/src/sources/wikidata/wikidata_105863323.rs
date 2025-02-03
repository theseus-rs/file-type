use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863323: FileFormat = FileFormat {
    id: 105_863_323,
    source_type: SourceType::Wikidata,
    name: "Quartet ST module",
    extensions: &["4q"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x51, 0x55, 0x41, 0x52, 0x54, 0x45, 0x54, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
