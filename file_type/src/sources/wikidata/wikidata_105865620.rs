use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865620: FileFormat = FileFormat {
    id: 105_865_620,
    source_type: SourceType::Wikidata,
    name: "Ashampoo Magical Security encrypted",
    extensions: &["ppenc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x53, 0x48, 0x50, 0x50, 0x45, 0x4E, 0x43, 0x45, 0x4E, 0x43, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
