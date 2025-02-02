use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863859: FileFormat = FileFormat {
    id: 105_863_859,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD animation (gen)",
    extensions: &["mot"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x4D, 0x4F, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
