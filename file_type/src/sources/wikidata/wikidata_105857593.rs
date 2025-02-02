use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857593: FileFormat = FileFormat {
    id: 105_857_593,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine Variables",
    extensions: &["var"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x39, 0x30, 0x32, 0x20, 0x20, 0x20, 0x20, 0x20, 0x4E, 0x6F, 0x5F, 0x41, 0x6E,
                    0x61, 0x72, 0x63, 0x68, 0x79, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
