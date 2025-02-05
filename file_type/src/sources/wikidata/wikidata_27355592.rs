use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27355592: FileFormat = FileFormat {
    id: 27_355_592,
    source_type: SourceType::Wikidata,
    name: "ADRG Geo Data File",
    extensions: &["img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
