use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855951: FileFormat = FileFormat {
    id: 105_855_951,
    source_type: SourceType::Wikidata,
    name: "DBC Communication Database for CAN (with rem)",
    extensions: &["dbc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4D, 0x5F, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
