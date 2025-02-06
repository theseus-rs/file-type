use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74551039: FileFormat = FileFormat {
    id: 74_551_039,
    source_type: SourceType::Wikidata,
    name: "ArcGIS geospatial and attribute data",
    extensions: &["sdc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x32, 0x43, 0x44, 0x53, 0x00, 0x02, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
