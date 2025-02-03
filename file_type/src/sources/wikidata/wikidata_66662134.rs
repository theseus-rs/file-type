use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66662134: FileFormat = FileFormat {
    id: 66_662_134,
    source_type: SourceType::Wikidata,
    name: "Lotus Word Pro SmartMaster",
    extensions: &["mwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
