use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855599: FileFormat = FileFormat {
    id: 105_855_599,
    source_type: SourceType::Wikidata,
    name: "Borland Overlay",
    extensions: &["ovr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x42, 0x4F, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
