use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858620: FileFormat = FileFormat {
    id: 105_858_620,
    source_type: SourceType::Wikidata,
    name: "Taxman's Retro Engine v5 Configuration",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x46, 0x47, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
