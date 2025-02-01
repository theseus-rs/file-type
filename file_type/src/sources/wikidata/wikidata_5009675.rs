use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5009675: FileFormat = FileFormat {
    id: 5_009_675,
    puid: "wikidata/5009675",
    name: "CCP4",
    extensions: &["ccp4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
