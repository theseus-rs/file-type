use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863856: FileFormat = FileFormat {
    id: 105_863_856,
    source_type: SourceType::Wikidata,
    name: "MPLAB IDE Project",
    extensions: &["mcp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
