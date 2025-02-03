use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855369: FileFormat = FileFormat {
    id: 105_855_369,
    source_type: SourceType::Wikidata,
    name: "WinFax Pro multipage document",
    extensions: &["fxm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0B, 0x23, 0xC8, 0xC0, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
