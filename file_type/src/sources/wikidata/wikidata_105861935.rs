use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861935: FileFormat = FileFormat {
    id: 105_861_935,
    puid: "wikidata/105861935",
    name: "Tecplot Macro",
    extensions: &["mcr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21, 0x4D, 0x43, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
