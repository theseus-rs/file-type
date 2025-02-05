use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850190: FileFormat = FileFormat {
    id: 105_850_190,
    source_type: SourceType::Wikidata,
    name: "Playmation Chor data",
    extensions: &["cho"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x4F, 0x52, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
