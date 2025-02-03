use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850963: FileFormat = FileFormat {
    id: 105_850_963,
    source_type: SourceType::Wikidata,
    name: "SuperJAM! Toot",
    extensions: &["toot"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4F, 0x4F, 0x54, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
