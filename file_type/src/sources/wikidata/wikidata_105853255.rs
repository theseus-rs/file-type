use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853255: FileFormat = FileFormat {
    id: 105_853_255,
    source_type: SourceType::Wikidata,
    name: "Dell System Information",
    extensions: &["sdr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x65, 0x6D])],
            },
        }],
    }],
    related_formats: &[],
};
