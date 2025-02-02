use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852676: FileFormat = FileFormat {
    id: 105_852_676,
    source_type: SourceType::Wikidata,
    name: "EditPlus Syntax file",
    extensions: &["stx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
