use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853674: FileFormat = FileFormat {
    id: 105_853_674,
    source_type: SourceType::Wikidata,
    name: "Address Book CoreData Person",
    extensions: &["abcdp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
