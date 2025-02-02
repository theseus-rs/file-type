use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852994: FileFormat = FileFormat {
    id: 105_852_994,
    source_type: SourceType::Wikidata,
    name: "SatcoDX channel list",
    extensions: &["sdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
