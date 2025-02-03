use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855751: FileFormat = FileFormat {
    id: 105_855_751,
    source_type: SourceType::Wikidata,
    name: "DeskSoft License",
    extensions: &["desksoftlicense"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x74, 0x5D, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
