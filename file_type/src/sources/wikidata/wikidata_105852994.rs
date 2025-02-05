use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852994: FileFormat = FileFormat {
    id: 105_852_994,
    source_type: SourceType::Wikidata,
    name: "SatcoDX channel list",
    extensions: &["sdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
