use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851089: FileFormat = FileFormat {
    id: 105_851_089,
    source_type: SourceType::Wikidata,
    name: "Textual Data Base (v2.x)",
    extensions: &["tdb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x44, 0x42, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
