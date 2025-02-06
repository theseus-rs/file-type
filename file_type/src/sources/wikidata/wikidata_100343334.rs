use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100343334: FileFormat = FileFormat {
    id: 100_343_334,
    source_type: SourceType::Wikidata,
    name: "Corel Print House/Print Office Document, version 5",
    extensions: &["cpd", "cph", "cpo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
