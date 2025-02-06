use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50374913: FileFormat = FileFormat {
    id: 50_374_913,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Design Web Format",
    extensions: &["dwfx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
