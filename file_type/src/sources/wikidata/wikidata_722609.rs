use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_722609: FileFormat = FileFormat {
    id: 722_609,
    puid: "wikidata/722609",
    name: "MARC standards",
    extensions: &["marc", "mrc"],
    media_types: &["application/marc", "application/marc"],
    internal_signatures: &[],
    related_formats: &[],
};
