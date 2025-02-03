use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852729: FileFormat = FileFormat {
    id: 105_852_729,
    source_type: SourceType::Wikidata,
    name: "Microsoft Serialized certificate Store",
    extensions: &["sst"],
    media_types: &["application/vnd.ms-pki.certstore"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x43, 0x45, 0x52, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
