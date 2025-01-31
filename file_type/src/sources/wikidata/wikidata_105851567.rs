use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851567: FileFormat = FileFormat {
    id: 105_851_567,
    puid: "wikidata/105851567",
    name: "Teapot XDR spreadsheet",
    extensions: &["tp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x21, 0x74, 0x65, 0x61, 0x70, 0x6F, 0x74, 0x0A, 0x78, 0x64, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
