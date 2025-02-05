use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858885: FileFormat = FileFormat {
    id: 105_858_885,
    source_type: SourceType::Wikidata,
    name: "GFA Raytrace Compressed image (hi-res) bitmap",
    extensions: &["sch"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x63, 0x68, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
