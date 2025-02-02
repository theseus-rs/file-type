use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857996: FileFormat = FileFormat {
    id: 105_857_996,
    source_type: SourceType::Wikidata,
    name: "DIV Games Studio Font Source",
    extensions: &["ifs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x46, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
