use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855804: FileFormat = FileFormat {
    id: 105_855_804,
    source_type: SourceType::Wikidata,
    name: "Ensemble Studios Data Resource",
    extensions: &["drs"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x28, 0x63, 0x29,
                    0x20, 0x31, 0x39, 0x39, 0x37, 0x20, 0x45, 0x6E, 0x73, 0x65, 0x6D, 0x62, 0x6C,
                    0x65, 0x20, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x73, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
