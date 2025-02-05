use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75597761: FileFormat = FileFormat {
    id: 75_597_761,
    source_type: SourceType::Wikidata,
    name: "GeoGebra format, version 1.x",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    signatures: &[],
    related_formats: &[],
};
