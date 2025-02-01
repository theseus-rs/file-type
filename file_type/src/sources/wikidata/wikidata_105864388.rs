use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864388: FileFormat = FileFormat {
    id: 105_864_388,
    puid: "wikidata/105864388",
    name: "RawTherapee Postprocessing Profile",
    extensions: &["pp3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
