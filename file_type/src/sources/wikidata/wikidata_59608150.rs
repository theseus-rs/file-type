use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59608150: FileFormat = FileFormat {
    id: 59_608_150,
    puid: "wikidata/59608150",
    name: "Microsoft Expression Media",
    extensions: &["ivc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
