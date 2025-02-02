use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859951: FileFormat = FileFormat {
    id: 105_859_951,
    source_type: SourceType::Wikidata,
    name: "Installer VISE Mac package (old)",
    extensions: &["vct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x49, 0x53, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
