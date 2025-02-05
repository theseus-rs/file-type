use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859321: FileFormat = FileFormat {
    id: 105_859_321,
    source_type: SourceType::Wikidata,
    name: "QML Cached document",
    extensions: &["qmlc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x71, 0x76, 0x34, 0x63, 0x64, 0x61, 0x74, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
