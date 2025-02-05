use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121788559: FileFormat = FileFormat {
    id: 121_788_559,
    source_type: SourceType::Wikidata,
    name: "Leapfrog Geo 3D Scene Format",
    extensions: &["lfsc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
