use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855711: FileFormat = FileFormat {
    id: 105_855_711,
    puid: "wikidata/105855711",
    name: "Open Scenegraph scene",
    extensions: &["osg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x72, 0x6F, 0x75, 0x70, 0x20, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
