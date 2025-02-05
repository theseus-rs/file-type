use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855404: FileFormat = FileFormat {
    id: 105_855_404,
    source_type: SourceType::Wikidata,
    name: "foobar2000 component",
    extensions: &["fb2k-component"],
    media_types: &["application/zip"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
