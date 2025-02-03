use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850623: FileFormat = FileFormat {
    id: 105_850_623,
    source_type: SourceType::Wikidata,
    name: "Atari Control Panel applet",
    extensions: &["cpx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x64, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
