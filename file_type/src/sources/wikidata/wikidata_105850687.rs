use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850687: FileFormat = FileFormat {
    id: 105_850_687,
    source_type: SourceType::Wikidata,
    name: "MapInfo Sea Chart",
    extensions: &["kap"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21])],
            },
        }],
    }],
    related_formats: &[],
};
