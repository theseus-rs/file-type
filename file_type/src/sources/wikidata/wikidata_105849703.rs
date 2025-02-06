use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849703: FileFormat = FileFormat {
    id: 105_849_703,
    source_type: SourceType::Wikidata,
    name: "KOMPAS Drawing",
    extensions: &["cdw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
