use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975865: FileFormat = FileFormat {
    id: 28_975_865,
    source_type: SourceType::Wikidata,
    name: "OOGL VECT file",
    extensions: &["vect"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
