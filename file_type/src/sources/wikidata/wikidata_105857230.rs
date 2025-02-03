use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857230: FileFormat = FileFormat {
    id: 105_857_230,
    source_type: SourceType::Wikidata,
    name: "StarWriter for MS-DOS Graphics video Driver",
    extensions: &["hgd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x47, 0x44, 0x20, 0x30, 0x32, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
