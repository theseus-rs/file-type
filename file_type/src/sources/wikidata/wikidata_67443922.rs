use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67443922: FileFormat = FileFormat {
    id: 67_443_922,
    puid: "wikidata/67443922",
    name: "Black and White 2 Environment data",
    extensions: &["bwe"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
