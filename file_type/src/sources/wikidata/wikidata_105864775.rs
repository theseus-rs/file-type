use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864775: FileFormat = FileFormat {
    id: 105_864_775,
    source_type: SourceType::Wikidata,
    name: "SilkExplorer - Performance Explorer Workspace",
    extensions: &["pex"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0B, 0x00, 0x50, 0x45, 0x57, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
