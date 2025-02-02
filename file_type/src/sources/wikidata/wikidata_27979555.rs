use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979555: FileFormat = FileFormat {
    id: 27_979_555,
    source_type: SourceType::Wikidata,
    name: "Civilization III BIX saved game format",
    extensions: &["bix"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x43, 0x58, 0x56, 0x45, 0x52, 0x23,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
