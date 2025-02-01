use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865884: FileFormat = FileFormat {
    id: 105_865_884,
    puid: "wikidata/105865884",
    name: "Spectrum Pro Sound Maker chiptune",
    extensions: &["psm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x73, 0x6D, 0x31, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
