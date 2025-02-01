use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856163: FileFormat = FileFormat {
    id: 105_856_163,
    puid: "wikidata/105856163",
    name: "FL Studio Drum Patch (generic)",
    extensions: &["dmpatch"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
