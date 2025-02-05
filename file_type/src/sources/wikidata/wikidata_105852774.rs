use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852774: FileFormat = FileFormat {
    id: 105_852_774,
    source_type: SourceType::Wikidata,
    name: "ESPS Sampled Data",
    extensions: &["sd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x6A, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
