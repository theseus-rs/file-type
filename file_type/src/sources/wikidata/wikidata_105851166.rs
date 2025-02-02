use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851166: FileFormat = FileFormat {
    id: 105_851_166,
    source_type: SourceType::Wikidata,
    name: "Track Record Viewer TRV/TRVX definition",
    extensions: &["trv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28, 0x43, 0x46, 0x47, 0x29, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
