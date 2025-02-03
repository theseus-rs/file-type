use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859116: FileFormat = FileFormat {
    id: 105_859_116,
    source_type: SourceType::Wikidata,
    name: "Award BIOS logo bitmap (136x84) (v1)",
    extensions: &["epa"],
    media_types: &["image/x-award-bioslogo"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x11, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
