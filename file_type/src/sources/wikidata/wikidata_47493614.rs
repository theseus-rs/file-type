use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47493614: FileFormat = FileFormat {
    id: 47_493_614,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS3",
    extensions: &["ind", "indd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
