use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860979: FileFormat = FileFormat {
    id: 105_860_979,
    source_type: SourceType::Wikidata,
    name: "LabVIEW Measurement",
    extensions: &["lvm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x61, 0x62, 0x56, 0x49, 0x45, 0x57, 0x20, 0x4D, 0x65, 0x61, 0x73, 0x75,
                    0x72, 0x65, 0x6D, 0x65, 0x6E, 0x74, 0x09, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
