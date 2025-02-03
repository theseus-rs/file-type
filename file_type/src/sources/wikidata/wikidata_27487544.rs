use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27487544: FileFormat = FileFormat {
    id: 27_487_544,
    source_type: SourceType::Wikidata,
    name: "Shapefile codepage file",
    extensions: &["cpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
