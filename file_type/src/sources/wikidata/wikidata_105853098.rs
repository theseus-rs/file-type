use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853098: FileFormat = FileFormat {
    id: 105_853_098,
    source_type: SourceType::Wikidata,
    name: "Spybot-S&D \"malware\" informations",
    extensions: &["sbi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
