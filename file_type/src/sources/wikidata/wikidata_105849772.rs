use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849772: FileFormat = FileFormat {
    id: 105_849_772,
    puid: "wikidata/105849772",
    name: "Cabal info",
    extensions: &["cabal"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x61, 0x6D, 0x65, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
