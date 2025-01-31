use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866510: FileFormat = FileFormat {
    id: 105_866_510,
    puid: "wikidata/105866510",
    name: "PhotoMovieMaker / Smilebox slideshow",
    extensions: &["pmm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x73, 0x6C, 0x69, 0x64, 0x65, 0x73, 0x68, 0x6F, 0x77, 0x20, 0x76, 0x65,
                    0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
