use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853224: FileFormat = FileFormat {
    id: 105_853_224,
    source_type: SourceType::Wikidata,
    name: "MSX Home Office Spreadsheet",
    extensions: &["ss1"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x53, 0x31, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
