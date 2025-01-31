use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27902177: FileFormat = FileFormat {
    id: 27_902_177,
    puid: "wikidata/27902177",
    name: "Amateur Data Interchange Format, ADX variant, version 3.0.2",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
