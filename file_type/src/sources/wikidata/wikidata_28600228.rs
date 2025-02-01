use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600228: FileFormat = FileFormat {
    id: 28_600_228,
    puid: "wikidata/28600228",
    name: "APL workspace",
    extensions: &["apl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
