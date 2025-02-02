use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857994: FileFormat = FileFormat {
    id: 105_857_994,
    source_type: SourceType::Wikidata,
    name: "ScummVM settings",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x73, 0x63, 0x75, 0x6D, 0x6D, 0x76, 0x6D, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
