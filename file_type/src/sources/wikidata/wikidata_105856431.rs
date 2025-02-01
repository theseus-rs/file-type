use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856431: FileFormat = FileFormat {
    id: 105_856_431,
    puid: "wikidata/105856431",
    name: "Total Commander Packer extension (plugin)",
    extensions: &["wcx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
