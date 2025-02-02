use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865176: FileFormat = FileFormat {
    id: 105_865_176,
    source_type: SourceType::Wikidata,
    name: "Back-It Preset (v4)",
    extensions: &["prs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x41, 0x5A, 0x45, 0x4C, 0x4C, 0x45, 0x20, 0x50, 0x52, 0x45, 0x53, 0x45,
                    0x54, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
