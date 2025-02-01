use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850438: FileFormat = FileFormat {
    id: 105_850_438,
    puid: "wikidata/105850438",
    name: "EM400 Configuration",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6D, 0x70, 0x75, 0x74, 0x65, 0x72, 0x20, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
