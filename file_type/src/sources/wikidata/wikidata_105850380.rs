use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850380: FileFormat = FileFormat {
    id: 105_850_380,
    source_type: SourceType::Wikidata,
    name: "MSX Home Office document/data",
    extensions: &["ccw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x43, 0x57, 0x53, 0x53, 0x53, 0x53, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
