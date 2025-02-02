use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853150: FileFormat = FileFormat {
    id: 105_853_150,
    source_type: SourceType::Wikidata,
    name: "Scriptable Network Graphics",
    extensions: &["sng"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x53, 0x4E, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
