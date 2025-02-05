use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850766: FileFormat = FileFormat {
    id: 105_850_766,
    source_type: SourceType::Wikidata,
    name: "Live For Speed Knowledge AI data",
    extensions: &["knw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x46, 0x53, 0x4B, 0x4E, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
