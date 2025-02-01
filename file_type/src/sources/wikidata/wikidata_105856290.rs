use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856290: FileFormat = FileFormat {
    id: 105_856_290,
    puid: "wikidata/105856290",
    name: "Dream Station 1.0 module",
    extensions: &["dss"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x53, 0x46, 0x6D, 0x74, 0x31, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
