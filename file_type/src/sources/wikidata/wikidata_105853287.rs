use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853287: FileFormat = FileFormat {
    id: 105_853_287,
    source_type: SourceType::Wikidata,
    name: "STereoLithography (ASCII)",
    extensions: &["stl"],
    media_types: &["model/x.stl-ascii"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x6F, 0x6C, 0x69, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
