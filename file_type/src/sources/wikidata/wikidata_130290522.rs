use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130290522: FileFormat = FileFormat {
    id: 130_290_522,
    source_type: SourceType::Wikidata,
    name: "Meson file format",
    extensions: &["meson.build"],
    media_types: &["text/x-meson"],
    signatures: &[],
    related_formats: &[],
};
