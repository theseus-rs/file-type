use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113773526: FileFormat = FileFormat {
    id: 113_773_526,
    source_type: SourceType::Wikidata,
    name: "Painter Raster Image",
    extensions: &["rif", "riff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
