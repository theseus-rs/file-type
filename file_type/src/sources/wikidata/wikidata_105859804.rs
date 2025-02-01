use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859804: FileFormat = FileFormat {
    id: 105_859_804,
    puid: "wikidata/105859804",
    name: "Eyemail video",
    extensions: &["eye"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x65, 0x79, 0x65, 0x6D, 0x61, 0x69, 0x6C, 0x76, 0x69, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
