use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855310: FileFormat = FileFormat {
    id: 105_855_310,
    source_type: SourceType::Wikidata,
    name: "FM-Kingtracker module",
    extensions: &["fmk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4D, 0x4B, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
