use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856717: FileFormat = FileFormat {
    id: 105_856_717,
    puid: "wikidata/105856717",
    name: "X-ray diffraction data file format",
    extensions: &["udf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x49, 0x64, 0x65, 0x6E, 0x74, 0x2C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
