use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5227180: FileFormat = FileFormat {
    id: 5_227_180,
    puid: "wikidata/5227180",
    name: "Data Interchange Format",
    extensions: &["dif"],
    media_types: &["application/x-dif"],
    internal_signatures: &[],
    related_formats: &[],
};
