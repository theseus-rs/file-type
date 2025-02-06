use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852564: FileFormat = FileFormat {
    id: 105_852_564,
    source_type: SourceType::Wikidata,
    name: "StarLogo project",
    extensions: &["slogo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x61, 0x76, 0x61, 0x20, 0x53, 0x74, 0x61, 0x72, 0x4C, 0x6F, 0x67, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
