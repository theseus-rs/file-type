use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826340: FileFormat = FileFormat {
    id: 27_826_340,
    source_type: SourceType::Wikidata,
    name: "Auxiliary file, AUX variant",
    extensions: &["aux"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
