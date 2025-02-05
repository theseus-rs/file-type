use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866915: FileFormat = FileFormat {
    id: 105_866_915,
    source_type: SourceType::Wikidata,
    name: "NWiper show",
    extensions: &["nw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x57, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
