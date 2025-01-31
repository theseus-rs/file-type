use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857857: FileFormat = FileFormat {
    id: 105_857_857,
    puid: "wikidata/105857857",
    name: "INDENICA Variability Modelling Language",
    extensions: &["ivml"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
