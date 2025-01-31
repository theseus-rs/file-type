use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855965: FileFormat = FileFormat {
    id: 105_855_965,
    puid: "wikidata/105855965",
    name: "FL Studio Drum Patch (v2)",
    extensions: &["dmpatch"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x50, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
