use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850578: FileFormat = FileFormat {
    id: 105_850_578,
    source_type: SourceType::Wikidata,
    name: "Sietronics CPI XRD format",
    extensions: &["cpi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x45, 0x54, 0x52, 0x4F, 0x4E, 0x49, 0x43, 0x53, 0x20, 0x58, 0x52,
                    0x44, 0x20, 0x53, 0x43, 0x41, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
