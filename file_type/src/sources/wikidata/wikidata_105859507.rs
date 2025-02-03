use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859507: FileFormat = FileFormat {
    id: 105_859_507,
    source_type: SourceType::Wikidata,
    name: "Genetec Video Archive",
    extensions: &["g64"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x65, 0x6E, 0x65, 0x74, 0x65, 0x63, 0x20, 0x4F, 0x6D, 0x6E, 0x69, 0x63,
                    0x61, 0x73, 0x74, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
