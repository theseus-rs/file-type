use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856177: FileFormat = FileFormat {
    id: 105_856_177,
    source_type: SourceType::Wikidata,
    name: "DBC Communication Database for CAN",
    extensions: &["dbc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x22, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
