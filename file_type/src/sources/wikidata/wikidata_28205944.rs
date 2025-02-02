use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205944: FileFormat = FileFormat {
    id: 28_205_944,
    source_type: SourceType::Wikidata,
    name: "Doré Raster",
    extensions: &["dore", "img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
