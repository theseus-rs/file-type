use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853238: FileFormat = FileFormat {
    id: 105_853_238,
    puid: "wikidata/105853238",
    name: "Scidb player/event/site/annotator data",
    extensions: &["scn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x63, 0x69, 0x64, 0x62, 0x2E, 0x6E, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
