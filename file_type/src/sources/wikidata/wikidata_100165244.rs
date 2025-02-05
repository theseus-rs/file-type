use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100165244: FileFormat = FileFormat {
    id: 100_165_244,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Format (ASCII), version 2018/2019/2020",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
