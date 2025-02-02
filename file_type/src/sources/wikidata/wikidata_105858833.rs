use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858833: FileFormat = FileFormat {
    id: 105_858_833,
    source_type: SourceType::Wikidata,
    name: "Signum! bitmap",
    extensions: &["imc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x62, 0x69, 0x6D, 0x63, 0x30, 0x30, 0x30, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
