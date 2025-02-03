use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857175: FileFormat = FileFormat {
    id: 105_857_175,
    source_type: SourceType::Wikidata,
    name: "Genieous Snapshot",
    extensions: &["hts"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x53, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
