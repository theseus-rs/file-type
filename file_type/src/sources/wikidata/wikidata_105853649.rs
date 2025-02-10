use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853649: FileFormat = FileFormat {
    id: 105_853_649,
    source_type: SourceType::Wikidata,
    name: "Authorware Packaged file (w/o runtime)",
    extensions: &["a4p", "a5p"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x52, 0x53, 0xBE, 0xBC, 0xAD, 0xAC, 0x16,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
