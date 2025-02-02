use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60887256: FileFormat = FileFormat {
    id: 60_887_256,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 1.1",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
