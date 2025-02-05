use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100344893: FileFormat = FileFormat {
    id: 100_344_893,
    source_type: SourceType::Wikidata,
    name: "Corel Photo House Image",
    extensions: &["cps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
