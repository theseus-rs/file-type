use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866853: FileFormat = FileFormat {
    id: 105_866_853,
    puid: "wikidata/105866853",
    name: "PSG chiptune",
    extensions: &["psg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x47, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
