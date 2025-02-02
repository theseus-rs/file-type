use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856992: FileFormat = FileFormat {
    id: 105_856_992,
    source_type: SourceType::Wikidata,
    name: "GeoplanW data",
    extensions: &["g2w"],
    media_types: &["application/vnd.geoplan"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x69, 0x67, 0x75, 0x72, 0x65, 0x20, 0x47, 0xE9, 0x6F, 0x70, 0x6C, 0x61,
                    0x6E, 0x0D, 0x0A, 0x4E, 0x75, 0x6D, 0xE9, 0x72, 0x6F, 0x20, 0x64, 0x65, 0x20,
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
