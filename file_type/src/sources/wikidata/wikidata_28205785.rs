use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205785: FileFormat = FileFormat {
    id: 28_205_785,
    source_type: SourceType::Wikidata,
    name: "BRender PIX",
    extensions: &["pix"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00,
                    0x00, 0x00, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
