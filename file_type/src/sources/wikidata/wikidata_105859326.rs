use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859326: FileFormat = FileFormat {
    id: 105_859_326,
    source_type: SourceType::Wikidata,
    name: "Qmage encoded data",
    extensions: &["qmg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
