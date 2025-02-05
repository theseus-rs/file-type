use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856495: FileFormat = FileFormat {
    id: 105_856_495,
    source_type: SourceType::Wikidata,
    name: "Visual Studio Web Deployment Project",
    extensions: &["wdproj"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x21, 0x2D, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
