use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859181: FileFormat = FileFormat {
    id: 105_859_181,
    puid: "wikidata/105859181",
    name: "BibTeX Generated Bibliography",
    extensions: &["bbl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5C, 0x62, 0x65, 0x67, 0x69, 0x6E, 0x7B, 0x74, 0x68, 0x65, 0x62, 0x69, 0x62,
                    0x6C, 0x69, 0x6F, 0x67, 0x72, 0x61, 0x70, 0x68, 0x79, 0x7D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
