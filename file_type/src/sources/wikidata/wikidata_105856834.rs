use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856834: FileFormat = FileFormat {
    id: 105_856_834,
    source_type: SourceType::Wikidata,
    name: "HEC-RAS Geometry file",
    extensions: &["g01", "g02", "g99"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x65, 0x6F, 0x6D, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
