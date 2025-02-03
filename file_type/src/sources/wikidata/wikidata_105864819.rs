use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864819: FileFormat = FileFormat {
    id: 105_864_819,
    source_type: SourceType::Wikidata,
    name: "Poser scene",
    extensions: &["pz3"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x0A, 0x0A, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x0A, 0x09, 0x7B,
                    0x0A, 0x09, 0x6E, 0x75, 0x6D, 0x62, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
