use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857083: FileFormat = FileFormat {
    id: 105_857_083,
    source_type: SourceType::Wikidata,
    name: "G-code (generated by Cura)",
    extensions: &["gcode"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
