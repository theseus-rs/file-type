use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121784289: FileFormat = FileFormat {
    id: 121_784_289,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator CC 2020 Artwork 24-24.1",
    extensions: &["ai", "ait"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
