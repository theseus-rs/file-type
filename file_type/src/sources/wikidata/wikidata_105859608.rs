use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859608: FileFormat = FileFormat {
    id: 105_859_608,
    source_type: SourceType::Wikidata,
    name: "FutureVision FST video",
    extensions: &["fst"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x54, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
