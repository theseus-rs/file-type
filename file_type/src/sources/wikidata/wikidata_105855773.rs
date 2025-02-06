use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855773: FileFormat = FileFormat {
    id: 105_855_773,
    source_type: SourceType::Wikidata,
    name: "DoReMIX song",
    extensions: &["dmx"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x20, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
