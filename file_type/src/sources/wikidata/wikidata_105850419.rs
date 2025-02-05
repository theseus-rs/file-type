use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850419: FileFormat = FileFormat {
    id: 105_850_419,
    source_type: SourceType::Wikidata,
    name: "CryoGen ECC data",
    extensions: &["cyg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x79, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
