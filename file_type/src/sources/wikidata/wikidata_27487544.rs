use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487544: FileFormat = FileFormat {
    id: 27_487_544,
    source_type: SourceType::Wikidata,
    name: "Shapefile codepage file",
    extensions: &["cpg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
