use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851625: FileFormat = FileFormat {
    id: 105_851_625,
    source_type: SourceType::Wikidata,
    name: "Fugawi Tracklog file",
    extensions: &["trk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x55, 0x47, 0x54, 0x52, 0x4B, 0xFF, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
