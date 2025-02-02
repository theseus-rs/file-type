use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100343334: FileFormat = FileFormat {
    id: 100_343_334,
    source_type: SourceType::Wikidata,
    name: "Corel Print House/Print Office Document, version 5",
    extensions: &["cpd", "cph", "cpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
