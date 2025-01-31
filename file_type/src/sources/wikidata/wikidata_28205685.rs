use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205685: FileFormat = FileFormat {
    id: 28_205_685,
    puid: "wikidata/28205685",
    name: "AMOS Picture Bank",
    extensions: &["abk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
