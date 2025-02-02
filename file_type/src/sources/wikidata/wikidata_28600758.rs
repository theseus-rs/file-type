use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600758: FileFormat = FileFormat {
    id: 28_600_758,
    source_type: SourceType::Wikidata,
    name: "ETC Show File",
    extensions: &["shw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x54, 0x43, 0x20, 0x45, 0x58, 0x50, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
