use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61752184: FileFormat = FileFormat {
    id: 61_752_184,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS",
    extensions: &["ind", "indd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
