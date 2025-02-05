use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853865: FileFormat = FileFormat {
    id: 105_853_865,
    source_type: SourceType::Wikidata,
    name: "Ventrilo audio recording",
    extensions: &["vrf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x45, 0x4E, 0x54, 0x52, 0x45, 0x43, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
