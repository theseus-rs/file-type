use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853340: FileFormat = FileFormat {
    id: 105_853_340,
    source_type: SourceType::Wikidata,
    name: "Maddox Games data archive",
    extensions: &["sfs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x46, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
