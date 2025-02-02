use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856055: FileFormat = FileFormat {
    id: 105_856_055,
    source_type: SourceType::Wikidata,
    name: "Inno Setup Uninstall Log (generic)",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x6E, 0x6E, 0x6F, 0x20, 0x53, 0x65, 0x74, 0x75, 0x70, 0x20, 0x55, 0x6E,
                    0x69, 0x6E, 0x73, 0x74, 0x61, 0x6C, 0x6C, 0x20, 0x4C, 0x6F, 0x67, 0x20, 0x28,
                    0x62, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
