use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919058: FileFormat = FileFormat {
    id: 28_919_058,
    puid: "wikidata/28919058",
    name: "Adobe Premiere Title",
    extensions: &["ptl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x54, 0x54, 0x4C, 0x35, 0x5D, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
