use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487495: FileFormat = FileFormat {
    id: 27_487_495,
    source_type: SourceType::Wikidata,
    name: "Shapefile spatial index of features part 2",
    extensions: &["fbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
