use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865121: FileFormat = FileFormat {
    id: 105_865_121,
    source_type: SourceType::Wikidata,
    name: "World Construction Set Preferences",
    extensions: &["prefs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x43, 0x53, 0x4D, 0x56, 0x50, 0x72, 0x65, 0x66, 0x73, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
