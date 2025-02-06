use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113773526: FileFormat = FileFormat {
    id: 113_773_526,
    source_type: SourceType::Wikidata,
    name: "Painter Raster Image",
    extensions: &["rif", "riff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
