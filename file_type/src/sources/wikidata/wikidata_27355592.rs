use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27355592: FileFormat = FileFormat {
    id: 27_355_592,
    source_type: SourceType::Wikidata,
    name: "ADRG Geo Data File",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
