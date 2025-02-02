use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851915: FileFormat = FileFormat {
    id: 105_851_915,
    source_type: SourceType::Wikidata,
    name: "WinAPE Snapshot (generic)",
    extensions: &["sna"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x56, 0x20, 0x2D, 0x20, 0x53, 0x4E, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
