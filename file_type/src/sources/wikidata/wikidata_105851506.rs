use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851506: FileFormat = FileFormat {
    id: 105_851_506,
    source_type: SourceType::Wikidata,
    name: "Vallen Transient Recorder data",
    extensions: &["tra"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x61, 0x6C, 0x6C, 0x65, 0x6E, 0x20, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x69,
                    0x65, 0x6E, 0x74, 0x20, 0x52, 0x65, 0x63, 0x6F, 0x72, 0x64, 0x65, 0x72, 0x20,
                    0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
