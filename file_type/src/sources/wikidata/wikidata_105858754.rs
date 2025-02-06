use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858754: FileFormat = FileFormat {
    id: 105_858_754,
    source_type: SourceType::Wikidata,
    name: "64 Colors BitMap",
    extensions: &["cbm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x42, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
