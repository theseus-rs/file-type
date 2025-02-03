use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979387: FileFormat = FileFormat {
    id: 27_979_387,
    source_type: SourceType::Wikidata,
    name: "Nero Super Video CD compilation",
    extensions: &["nsd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x45, 0x53, 0x44, 0x1A, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
