use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858458: FileFormat = FileFormat {
    id: 105_858_458,
    puid: "wikidata/105858458",
    name: "Epanet data file",
    extensions: &["net"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0x09, 0x3C, 0x45, 0x50, 0x41, 0x4E, 0x45, 0x54, 0x32, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
