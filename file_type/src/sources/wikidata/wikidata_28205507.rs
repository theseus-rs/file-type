use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205507: FileFormat = FileFormat {
    id: 28_205_507,
    source_type: SourceType::Wikidata,
    name: "GlowIcons",
    extensions: &["info"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE3, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
