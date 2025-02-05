use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_99844735: FileFormat = FileFormat {
    id: 99_844_735,
    source_type: SourceType::Wikidata,
    name: "GDAL Vector Virtual Format",
    extensions: &["vrt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
