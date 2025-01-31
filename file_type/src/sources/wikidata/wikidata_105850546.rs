use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850546: FileFormat = FileFormat {
    id: 105_850_546,
    puid: "wikidata/105850546",
    name: "World Construction Set Cloud map",
    extensions: &["cld"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x43, 0x53, 0x43, 0x6C, 0x6F, 0x75, 0x64, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
