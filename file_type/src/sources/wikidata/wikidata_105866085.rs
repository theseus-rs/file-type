use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866085: FileFormat = FileFormat {
    id: 105_866_085,
    puid: "wikidata/105866085",
    name: "PVM3 Volume format",
    extensions: &["pvm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x4D, 0x33, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
