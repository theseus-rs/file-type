use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855681: FileFormat = FileFormat {
    id: 105_855_681,
    source_type: SourceType::Wikidata,
    name: "Csound Orchestra",
    extensions: &["orc"],
    media_types: &["audio/csound"],
    signatures: &[Signature {
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
