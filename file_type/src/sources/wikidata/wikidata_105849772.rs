use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849772: FileFormat = FileFormat {
    id: 105_849_772,
    source_type: SourceType::Wikidata,
    name: "Cabal info",
    extensions: &["cabal"],
    media_types: &["text/plain"],
    signatures: &[Signature {
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
