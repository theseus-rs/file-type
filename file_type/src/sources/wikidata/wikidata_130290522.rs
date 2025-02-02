use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130290522: FileFormat = FileFormat {
    id: 130_290_522,
    source_type: SourceType::Wikidata,
    name: "Meson file format",
    extensions: &["meson.build"],
    media_types: &["text/x-meson"],
    internal_signatures: &[],
    related_formats: &[],
};
