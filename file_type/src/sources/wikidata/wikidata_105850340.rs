use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850340: FileFormat = FileFormat {
    id: 105_850_340,
    puid: "wikidata/105850340",
    name: "SuperCalc 2/3 spreadsheet (v1.00)",
    extensions: &["cal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x75, 0x70, 0x65, 0x72, 0x43, 0x61, 0x6C, 0x63, 0x20, 0x76, 0x65, 0x72,
                    0x2E, 0x20, 0x20, 0x31, 0x2E, 0x30, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
