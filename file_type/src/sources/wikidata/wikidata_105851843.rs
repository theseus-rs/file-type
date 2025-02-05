use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851843: FileFormat = FileFormat {
    id: 105_851_843,
    source_type: SourceType::Wikidata,
    name: "Spektrum DX serie transmitter settings",
    extensions: &["spm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x70, 0x65, 0x6B, 0x74, 0x72, 0x75, 0x6D, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
