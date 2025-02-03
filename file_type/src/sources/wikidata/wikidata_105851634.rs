use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851634: FileFormat = FileFormat {
    id: 105_851_634,
    source_type: SourceType::Wikidata,
    name: "Track Row Column markers data format",
    extensions: &["trc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x74, 0x68, 0x46, 0x69, 0x6C, 0x65, 0x54, 0x79, 0x70, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
