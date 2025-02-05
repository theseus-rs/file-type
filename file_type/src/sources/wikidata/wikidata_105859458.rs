use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859458: FileFormat = FileFormat {
    id: 105_859_458,
    source_type: SourceType::Wikidata,
    name: "dBASE compiled Query",
    extensions: &["qbo"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x05, 0x44, 0x42, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
