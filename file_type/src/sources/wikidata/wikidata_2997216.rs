use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2997216: FileFormat = FileFormat {
    id: 2_997_216,
    source_type: SourceType::Wikidata,
    name: "Core Audio Format",
    extensions: &["caf"],
    media_types: &["audio/x-caf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x61, 0x66, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
