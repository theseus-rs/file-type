use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75710135: FileFormat = FileFormat {
    id: 75_710_135,
    source_type: SourceType::Wikidata,
    name: "GeoGebra format, version 4",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    signatures: &[],
    related_formats: &[],
};
