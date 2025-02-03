use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866209: FileFormat = FileFormat {
    id: 105_866_209,
    source_type: SourceType::Wikidata,
    name: "PC-BSD Installer Package",
    extensions: &["pbi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0x45, 0x4C, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
