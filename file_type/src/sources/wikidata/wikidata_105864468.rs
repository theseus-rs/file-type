use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864468: FileFormat = FileFormat {
    id: 105_864_468,
    source_type: SourceType::Wikidata,
    name: "PalmDOC text document",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x45, 0x58, 0x74, 0x52, 0x45, 0x41, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
