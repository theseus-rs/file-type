use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849855: FileFormat = FileFormat {
    id: 105_849_855,
    source_type: SourceType::Wikidata,
    name: "Cal3D material File",
    extensions: &["crf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x52, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
