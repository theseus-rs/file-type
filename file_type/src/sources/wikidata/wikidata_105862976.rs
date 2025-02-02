use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862976: FileFormat = FileFormat {
    id: 105_862_976,
    source_type: SourceType::Wikidata,
    name: "Mystic BBS install package",
    extensions: &["mys"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x04, 0x4D, 0x59, 0x53, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
