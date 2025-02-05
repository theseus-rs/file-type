use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852485: FileFormat = FileFormat {
    id: 105_852_485,
    source_type: SourceType::Wikidata,
    name: "ArcGIS spatial and attribute indexes",
    extensions: &["sdi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x32, 0x49, 0x44, 0x53, 0x00, 0x02, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
