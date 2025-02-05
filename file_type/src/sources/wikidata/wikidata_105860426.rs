use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860426: FileFormat = FileFormat {
    id: 105_860_426,
    source_type: SourceType::Wikidata,
    name: "Nintendo Entertainment System ROM",
    extensions: &["nes"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x45, 0x53, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
