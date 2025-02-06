use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125822754: FileFormat = FileFormat {
    id: 125_822_754,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Index",
    extensions: &["chi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
