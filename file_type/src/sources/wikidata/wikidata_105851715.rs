use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851715: FileFormat = FileFormat {
    id: 105_851_715,
    puid: "wikidata/105851715",
    name: "XnView Slideshow Data",
    extensions: &["sld"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x53, 0x6C, 0x69, 0x64, 0x65, 0x20, 0x53, 0x68, 0x6F, 0x77, 0x20,
                    0x53, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x63, 0x65, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
