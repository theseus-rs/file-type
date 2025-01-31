use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000712: FileFormat = FileFormat {
    id: 29_000_712,
    puid: "wikidata/29000712",
    name: "TecPlot ASCII",
    extensions: &["tp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
