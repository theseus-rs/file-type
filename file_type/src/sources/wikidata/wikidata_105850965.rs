use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850965: FileFormat = FileFormat {
    id: 105_850_965,
    puid: "wikidata/105850965",
    name: "Binary Unicode conversion Table",
    extensions: &["tbl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x6E, 0x69, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
