use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27902219: FileFormat = FileFormat {
    id: 27_902_219,
    puid: "wikidata/27902219",
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.3",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
