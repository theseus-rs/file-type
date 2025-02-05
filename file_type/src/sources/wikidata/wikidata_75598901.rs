use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75598901: FileFormat = FileFormat {
    id: 75_598_901,
    source_type: SourceType::Wikidata,
    name: "GeoGebra format, version 3",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    signatures: &[],
    related_formats: &[],
};
