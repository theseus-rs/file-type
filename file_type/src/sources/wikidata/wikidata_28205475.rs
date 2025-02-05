use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205475: FileFormat = FileFormat {
    id: 28_205_475,
    source_type: SourceType::Wikidata,
    name: "ZoomBrowser Ex thumbnail cache",
    extensions: &["info"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7A, 0x62, 0x65, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
