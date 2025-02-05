use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850149: FileFormat = FileFormat {
    id: 105_850_149,
    source_type: SourceType::Wikidata,
    name: "LabVIEW Control data",
    extensions: &["ctl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x53, 0x52, 0x43, 0x0D, 0x0A, 0x00, 0x03, 0x4C, 0x56, 0x43, 0x43, 0x4C,
                    0x42, 0x56, 0x57,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
