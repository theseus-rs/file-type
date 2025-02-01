use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867272: FileFormat = FileFormat {
    id: 105_867_272,
    puid: "wikidata/105867272",
    name: "Legend text",
    extensions: &["nbi"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
