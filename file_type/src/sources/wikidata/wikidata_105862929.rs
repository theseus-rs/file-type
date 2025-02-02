use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862929: FileFormat = FileFormat {
    id: 105_862_929,
    source_type: SourceType::Wikidata,
    name: "MapWindow Project (v5)",
    extensions: &["mwproj"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x61, 0x70, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x35,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
