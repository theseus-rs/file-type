use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857265: FileFormat = FileFormat {
    id: 105_857_265,
    source_type: SourceType::Wikidata,
    name: "Hollywood Applet",
    extensions: &["hwa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x57, 0x5A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
