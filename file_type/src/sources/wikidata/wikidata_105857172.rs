use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857172: FileFormat = FileFormat {
    id: 105_857_172,
    source_type: SourceType::Wikidata,
    name: "HelpSmith Project",
    extensions: &["hsm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x53, 0x50, 0x52, 0x4F, 0x4A])],
            },
        }],
    }],
    related_formats: &[],
};
