use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66662134: FileFormat = FileFormat {
    id: 66_662_134,
    puid: "wikidata/66662134",
    name: "Lotus Word Pro SmartMaster",
    extensions: &["mwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
