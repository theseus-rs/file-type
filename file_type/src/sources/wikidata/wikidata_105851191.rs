use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851191: FileFormat = FileFormat {
    id: 105_851_191,
    source_type: SourceType::Wikidata,
    name: "StarOffice Gallery theme",
    extensions: &["thm"],
    media_types: &["application/x-stargallery-thm"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x04, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
