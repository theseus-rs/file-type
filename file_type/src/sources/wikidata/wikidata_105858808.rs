use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858808: FileFormat = FileFormat {
    id: 105_858_808,
    source_type: SourceType::Wikidata,
    name: "Business Plan data",
    extensions: &["bpd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x70, 0x6C, 0x61, 0x6E, 0x64, 0x61, 0x74, 0x61, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
