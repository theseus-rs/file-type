use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75710135: FileFormat = FileFormat {
    id: 75_710_135,
    puid: "wikidata/75710135",
    name: "GeoGebra format, version 4",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};
