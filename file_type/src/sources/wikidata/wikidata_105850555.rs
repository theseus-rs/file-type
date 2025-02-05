use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850555: FileFormat = FileFormat {
    id: 105_850_555,
    source_type: SourceType::Wikidata,
    name: "Chuck Biscuits/Black Artist module",
    extensions: &["cba"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x42, 0x41, 0xF9])],
            },
        }],
    }],
    related_formats: &[],
};
