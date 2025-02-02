use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131703746: FileFormat = FileFormat {
    id: 131_703_746,
    source_type: SourceType::Wikidata,
    name: "xRage hdf file",
    extensions: &["h5rage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
