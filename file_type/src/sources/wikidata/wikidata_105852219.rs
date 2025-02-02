use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852219: FileFormat = FileFormat {
    id: 105_852_219,
    source_type: SourceType::Wikidata,
    name: "MSX SCMD music",
    extensions: &["sdt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x53, 0x43, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
