use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73675958: FileFormat = FileFormat {
    id: 73_675_958,
    source_type: SourceType::Wikidata,
    name: "3M Printscape",
    extensions: &["psc", "std"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
