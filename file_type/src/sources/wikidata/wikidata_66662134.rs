use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66662134: FileFormat = FileFormat {
    id: 66_662_134,
    source_type: SourceType::Wikidata,
    name: "Lotus Word Pro SmartMaster",
    extensions: &["mwp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
