use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100343191: FileFormat = FileFormat {
    id: 100_343_191,
    source_type: SourceType::Wikidata,
    name: "Corel Print House/Print Office Document, version 4",
    extensions: &["cpd", "cph", "cpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
