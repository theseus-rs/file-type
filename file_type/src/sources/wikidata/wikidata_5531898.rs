use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5531898: FileFormat = FileFormat {
    id: 5_531_898,
    source_type: SourceType::Wikidata,
    name: "General Exchange Format",
    extensions: &["gxf"],
    media_types: &["application/gxf"],
    internal_signatures: &[],
    related_formats: &[],
};
