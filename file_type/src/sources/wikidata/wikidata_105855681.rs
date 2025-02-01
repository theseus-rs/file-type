use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855681: FileFormat = FileFormat {
    id: 105_855_681,
    puid: "wikidata/105855681",
    name: "Csound Orchestra",
    extensions: &["orc"],
    media_types: &["audio/csound"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x72, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
