use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849681: FileFormat = FileFormat {
    id: 105_849_681,
    source_type: SourceType::Wikidata,
    name: "Hatari Configuration",
    extensions: &["cfg"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x4C, 0x6F, 0x67, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
