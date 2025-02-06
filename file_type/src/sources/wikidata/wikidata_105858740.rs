use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858740: FileFormat = FileFormat {
    id: 105_858_740,
    source_type: SourceType::Wikidata,
    name: "packPNM compressed BMP bitmap",
    extensions: &["ppn"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
