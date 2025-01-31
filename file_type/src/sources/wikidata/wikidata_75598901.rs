use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75598901: FileFormat = FileFormat {
    id: 75_598_901,
    puid: "wikidata/75598901",
    name: "GeoGebra format, version 3",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};
