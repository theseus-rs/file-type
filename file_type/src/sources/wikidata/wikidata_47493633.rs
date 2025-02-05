use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47493633: FileFormat = FileFormat {
    id: 47_493_633,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS6",
    extensions: &["ind", "indd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
