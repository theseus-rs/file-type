use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47493628: FileFormat = FileFormat {
    id: 47_493_628,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS5",
    extensions: &["ind", "indd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
