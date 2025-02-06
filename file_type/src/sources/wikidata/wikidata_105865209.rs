use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865209: FileFormat = FileFormat {
    id: 105_865_209,
    source_type: SourceType::Wikidata,
    name: "Polyphonic Ringtone for Phones",
    extensions: &["pmd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x6D, 0x69, 0x64, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
