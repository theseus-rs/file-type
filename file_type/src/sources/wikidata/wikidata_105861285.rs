use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861285: FileFormat = FileFormat {
    id: 105_861_285,
    puid: "wikidata/105861285",
    name: "OpenTTD Language data",
    extensions: &["lng"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x41, 0x4E, 0x47, 0xCD, 0xED, 0x07, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
